use super::camera::{Camera, CameraController, Projection, uniform::CameraUniform};
use super::instance::{Instance, InstanceRaw};
use super::model::{DrawModel, Model, Vertex};

//temp
const NUM_INSTANCES_PER_ROW: u16 = 1;
use super::{model, resources, texture};
use cgmath::prelude::*;
use cgmath::{InnerSpace, Zero};
use pollster::block_on;
use std::iter;
use std::sync::Arc;
use std::time::Duration;
use wgpu::BufferSlice;
use wgpu::naga::FastHashMap;
use wgpu::util::DeviceExt;
use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;

pub struct State {
    pub mouse_pressed: bool,

    pub camera_controller: CameraController,
    //for model loading
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    //"new"
    projection: Projection,
    //new!
    pub clear_color: wgpu::Color,
    //stays
    pub surface: wgpu::Surface<'static>,
    //stays
    pub device: wgpu::Device,
    //stays
    pub queue: wgpu::Queue,
    //stays
    pub config: wgpu::SurfaceConfiguration,
    //stays
    pub is_surface_configured: bool,
    //stays
    pub render_pipeline: wgpu::RenderPipeline,
    //stays
    pub models: FastHashMap<&'static str, Model>,
    //stays very likely
    pub camera: Camera,
    //idk
    pub camera_uniform: CameraUniform,
    //we'll see
    pub camera_buffer: wgpu::Buffer,
    //same as above
    pub camera_bind_group: wgpu::BindGroup,
    //stays
    pub instances: Vec<Instance>,
    //for the vertex buffer, remove
    pub instance_buffer: wgpu::Buffer,
    //remove? - yes
    pub depth_texture: texture::Texture,
    // /\ replaces, only depth texture for now for easier usage
    pub depth_textures: Vec<texture::Texture>,
    //stays
    pub window: Arc<Window>,
    //pub obj_model: Model,
}

use crate::config::StateConfig;
use crate::errors::StateCreationError;
use crate::resources::load_model;

impl State {
    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, key: KeyCode, pressed: bool) {
        if !self.camera_controller.handle_key(key, pressed) {
            match (key, pressed) {
                (KeyCode::Escape, true) => event_loop.exit(),
                _ => {}
            }
        }
    }

    pub fn handle_mouse_button(&mut self, button: MouseButton, pressed: bool) {
        match button {
            MouseButton::Left => self.mouse_pressed = pressed,
            _ => {}
        }
    }

    // NEW!
    pub fn handle_mouse_scroll(&mut self, delta: &MouseScrollDelta) {
        self.camera_controller.handle_scroll(delta);
    }

    pub async fn new(
        window: Arc<Window>,
        general_config: StateConfig,
    ) -> Result<State, StateCreationError> {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        log::warn!("WGPU setup");
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .map_err(StateCreationError::RequestAdapterError)?;
        log::warn!("device and queue");
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .map_err(StateCreationError::RequestDeviceError)?;
        log::warn!("Surface");
        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an Srgb surface texture. Using a different
        // one will result all the colors comming out darker. If you want to support non
        // Srgb surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let surface_format = surface_format.remove_srgb_suffix();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        //TODO!: get it to the "distribution.rs" file
        const SPACE_BETWEEN: f32 = 3.0;
        let instances = (0..NUM_INSTANCES_PER_ROW)
            .flat_map(|z| {
                (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                    let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
                    let z = SPACE_BETWEEN * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);

                    let position = cgmath::Vector3 { x, y: 0.0, z };

                    let rotation = if position.is_zero() {
                        cgmath::Quaternion::from_axis_angle(
                            cgmath::Vector3::unit_z(),
                            cgmath::Deg(0.0),
                        )
                    } else {
                        cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
                    };

                    Instance { position, rotation }
                })
            })
            .collect::<Vec<_>>();

        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            //temporary, will be changed before the usage
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        //log::warn!("Load model");
        /*let obj_model = resources::load_model(
            "cube\\cube.obj",
            &device,
            &queue,
            &texture_bind_group_layout,
        )
        .await
        .map_err(StateCreationError::ModelError);*/

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader.wgsl"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let depth_texture =
            texture::Texture::create_depth_texture(&device, &config, "depth_texture");

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        //I only want to preset the capacity
        let mut models =
            FastHashMap::with_capacity_and_hasher(general_config.models.len(), Default::default());

        for (name, path) in general_config.models.into_iter() {
            let loaded = load_model(&path, &device, &queue, &texture_bind_group_layout)
                .await
                .map_err(StateCreationError::ModelError)?;
            models.insert(name, loaded);
        }

        //TODO!: configs!
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[model::ModelVertex::desc(), InstanceRaw::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                // or Features::POLYGON_MODE_POINT
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
            // Useful for optimizing shader compilation on Android
            cache: None,
        });
        let camera = Camera::new((0.0, 5.0, 10.0), cgmath::Deg(-90.0), cgmath::Deg(-20.0));

        let projection =
            Projection::new(config.width, config.height, cgmath::Deg(45.0), 0.1, 100.0);

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera, &projection);
        //TODO make configurable
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let clear_color = general_config.color;
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        camera_uniform.update_view_proj(&camera, &projection);

        Ok(Self {
            mouse_pressed: false,
            camera_controller: CameraController::new(4., 0.4),
            texture_bind_group_layout,
            projection,
            //TODO: temp
            depth_textures: Vec::new(),
            //obj_model: obj_model.unwrap(),
            //TODO
            surface,
            device,
            queue,
            config,
            clear_color,
            is_surface_configured: false,
            render_pipeline,
            //TODO!
            models,
            camera,
            camera_buffer,
            camera_bind_group,
            camera_uniform,
            instances,
            instance_buffer,
            depth_texture,
            window,
        })
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.projection.resize(width, height);
            self.is_surface_configured = true;
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.depth_texture =
                texture::Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
        }
    }

    //TODO!: refactor or remove and replace
    pub fn update(&mut self, dt: Duration) {
        self.camera_controller.update_camera(&mut self.camera, dt);
        //log::info!("{:?}", self.camera);
        self.camera_uniform
            .update_view_proj(&self.camera, &self.projection);
        log::info!("{:?}", self.camera_uniform);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    //TODO!: refactor!
    pub fn render(
        &mut self,
        model_ids: impl Iterator<Item = &'static str>,
    ) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();

        // We can't render unless the surface is configured
        if !self.is_surface_configured {
            return Ok(());
        }

        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        model_ids.for_each(|model_id| {
            //println!("Model ID: {}", model_id);
            //log::info!("rendering {:?}", model_id);

            //TODO: model existence guarantees?!

            //render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

            let model = &mut self.models.get(model_id).expect("???");

            render_pass.set_vertex_buffer(0, self.instance_buffer.slice(..));

            render_pass.set_pipeline(&self.render_pipeline);

            render_pass.draw_model(model, &self.camera_bind_group);
        });

        drop(render_pass);

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
