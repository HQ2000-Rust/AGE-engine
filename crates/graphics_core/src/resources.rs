use super::texture;
use crate::errors::{ModelError, TextureError};
use std::io::{BufReader, Cursor};
use std::path::Path;
use tobj::tokio as tobj_tokio;
use wgpu::util::DeviceExt;

use super::model;

pub async fn load_string(path: &Path) -> std::io::Result<String> {
    let txt = {
        //let path = Path::new("res").join(path);
        //assert!(path.exists(), "{} does not exist", path.display());
        std::fs::read_to_string(path)?
    };
    Ok(txt)
}

pub async fn load_binary(path: &Path) -> std::io::Result<Vec<u8>> {
    let data = {
        //let path = Path::new("res").join(path);
        //assert!(path.exists(), "{} does not exist", path.display());
        std::fs::read(path)?
    };

    Ok(data)
}

pub async fn load_texture(
    path: &Path,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> Result<texture::Texture, TextureError> {
    let data = load_binary(path).await.map_err(TextureError::IoError)?;
    //TODO!: no .unwrap()
    texture::Texture::from_bytes(
        device,
        queue,
        &data,
        path.file_name().unwrap().to_str().unwrap(),
    )
}

pub async fn load_model(
    file_name: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    layout: &wgpu::BindGroupLayout,
) -> Result<model::Model, ModelError> {
    let path = Path::new(file_name);
    let obj_text = load_string(path).await.map_err(ModelError::IoError)?;
    let obj_cursor = Cursor::new(obj_text);
    let mut obj_reader = tokio::io::BufReader::new(obj_cursor);

    let (models, obj_materials) = tobj_tokio::load_obj_buf(
        &mut obj_reader,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
        |p| async move {
            let mat_text =
                //make this more efficient later
                load_string(Path::new(path.parent().expect("TODO")).join(&p).as_ref())
                    .await
                    .map_err(|_e| tobj::LoadError::ReadError)?;
            tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
        },
    )
    .await
    .map_err(ModelError::LoadError)?;

    dbg!(&obj_materials.as_ref().unwrap().len());

    let mut materials = Vec::new();
    for m in obj_materials.map_err(ModelError::LoadError)? {
        //TODO!: remove .unwrap()
        let diffuse_texture = load_texture(
            Path::new(path.parent().expect("TODO"))
                .join(m.diffuse_texture.unwrap())
                .as_ref(),
            device,
            queue,
        )
        .await
        .map_err(ModelError::TextureError)?;
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: None,
        });

        materials.push(model::Material {
            name: m.name,
            diffuse_texture,
            bind_group,
        })
    }

    let meshes = models
        .into_iter()
        .map(|m| {
            let vertices = (0..m.mesh.positions.len() / 3)
                .map(|i| {
                    if m.mesh.normals.is_empty() {
                        model::ModelVertex {
                            position: [
                                m.mesh.positions[i * 3],
                                m.mesh.positions[i * 3 + 1],
                                m.mesh.positions[i * 3 + 2],
                            ],
                            tex_coords: [
                                m.mesh.texcoords[i * 2],
                                1.0 - m.mesh.texcoords[i * 2 + 1],
                            ],
                            normal: [0.0, 0.0, 0.0],
                        }
                    } else {
                        model::ModelVertex {
                            position: [
                                m.mesh.positions[i * 3],
                                m.mesh.positions[i * 3 + 1],
                                m.mesh.positions[i * 3 + 2],
                            ],
                            tex_coords: [
                                m.mesh.texcoords[i * 2],
                                1.0 - m.mesh.texcoords[i * 2 + 1],
                            ],
                            normal: [
                                m.mesh.normals[i * 3],
                                m.mesh.normals[i * 3 + 1],
                                m.mesh.normals[i * 3 + 2],
                            ],
                        }
                    }
                })
                .collect::<Vec<_>>();

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Vertex Buffer", file_name)),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Index Buffer", file_name)),
                contents: bytemuck::cast_slice(&m.mesh.indices),
                usage: wgpu::BufferUsages::INDEX,
            });

            log::info!("Mesh: {}", m.name);
            model::Mesh {
                name: file_name.to_string(),
                vertex_buffer,
                index_buffer,
                num_elements: m.mesh.indices.len() as u32,
                material: m.mesh.material_id.unwrap_or(0),
            }
        })
        .collect::<Vec<_>>();

    Ok(model::Model { meshes, materials })
}
