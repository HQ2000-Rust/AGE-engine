use crate::custom_events::builtin::ExitEvent;
use crate::events::EventHandle;
use crate::scheduler::Scheduler;
use anymap::{AnyMap, Map};
use graphics_core::state::State;
use std::convert::identity;
use std::hint::spin_loop;
use std::path::Path;

const FPS: u32 = 60;

pub struct Game {
    //Terminate is a ZST (=just symbolic)
    pub(crate) termination_tx: Option<std::sync::mpsc::SyncSender<Terminate>>,
    pub(crate) last_iteration: Instant,
    pub(crate) resources: SendAnyMap,
    pub(crate) scheduler: Scheduler,
    pub(crate) events: EventHandle,
    //only existent while setup
    pub(crate) graphics_state_config: Option<StateConfig>,
    pub(crate) graphics_state: Option<State>,
    //make it customisable later
    pub(crate) window_attributes: WindowAttributes,
    //TODO!: commands scheduler
    pub(crate) commands: Commands,
    pub(crate) world: World,
    //?
}

impl Default for Game {
    fn default() -> Self {
        Self {
            termination_tx: None,
            last_iteration: Instant::now(),
            resources: Map::new(),
            scheduler: Scheduler::new_empty(),
            events: EventHandle::new(),
            graphics_state: None,
            window_attributes: WindowAttributes::default(),
            commands: Commands::new(),
            graphics_state_config: Some(StateConfig::default()),
            world: World::new(),
        }
    }
}

enum CustomEvent {
    RunGameLogic,
}

struct Terminate;

impl Game {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_once(
        mut self,
        once: impl FnMut(&mut Commands, &mut SendAnyMap, &mut EventHandle) + 'static + Send,
    ) -> Self {
        self.scheduler.add_once(Box::new(once));
        self
    }

    pub fn add_update(
        mut self,
        update: impl FnMut(&mut Commands, &mut SendAnyMap, &mut EventHandle) + 'static + Send,
    ) -> Self {
        self.scheduler.add_update(Box::new(update));
        self
    }

    #[inline]
    pub fn prepare(&mut self) {}

    //fn exit(&mut self, event_loop: ActiveEventLoop) {}

    pub fn run(mut self) {
        warn!("Starting event loop");
        //TODO!: nicer error handling -> (inline) function
        let event_loop = EventLoop::with_user_event().build();

        match event_loop {
            Err(e) => {
                let message = format!("Fatal error while building the event loop: {}", e);
                error!("{}", message);
                panic!("{}", message);
            }
            Ok(event_loop) => {
                let proxy = event_loop.create_proxy();

                let (tx, rx) = std::sync::mpsc::sync_channel::<Terminate>(0);

                self.termination_tx = Some(tx);

                //clock
                std::thread::spawn(move || {
                    while rx.try_recv().is_err() {
                        let then = Instant::now();
                        if let Err(_) = proxy.send_event(CustomEvent::RunGameLogic) {
                            error!("Event loop doesn't exist anymore, terminating the clock");
                            break;
                        };

                        let time_to_sleep =
                            Instant::from(then + Duration::from_secs_f64(1. / FPS as f64))
                                .saturating_duration_since(Instant::now());

                        match time_to_sleep {
                            duration => std::thread::sleep(duration),
                            Duration::ZERO => {}
                        }
                    }
                    warn!("Clock terminating");
                    //optional termination logic
                });

                warn!("Preps finished");

                //let rwlock=RwLock::new(self);
                if let Err(e) = event_loop.run_app(&mut self) {
                    let message = format!("Fatal error while running the event loop: {}", e);
                    error!("{}", message);
                    panic!("{}", message);
                }
            }
        }
    }
}

impl ApplicationHandler<CustomEvent> for Game {
    //TODO!: make sure initialisation only happens once, resume() doesn't guarantee that!!
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.graphics_state.is_none() {
            event_loop.set_control_flow(ControlFlow::Poll);

            let window = Arc::new(
                event_loop
                    .create_window(self.window_attributes.clone())
                    .expect(
                        "Winit needs to be able to create a window, otherwise everything won't work",
                    ),
            );

            self.graphics_state = Some(
                pollster::block_on(State::new(
                    window,
                    self.graphics_state_config
                        .take()
                        .expect("Set to Some(_) before running the game"),
                ))
                //TODO: proper exception handling
                .expect("State creation needs to work"),
            );
        }

        //once logic

        self.scheduler
            .setup(&mut self.commands, &mut self.resources, &mut self.events);

        self.events.update();

        CommandHandle::update(self);
    }
    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: CustomEvent) {
        match event {
            CustomEvent::RunGameLogic => {}
        }

        self.scheduler
            .update(&mut self.commands, &mut self.resources, &mut self.events);

        self.events.update();
        CommandHandle::update(self);
    }
    //TODO
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        /*if event != WindowEvent::RedrawRequested {
            println!("{:?}", event);
        }*/

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            //I should refactor that later - TODO
            WindowEvent::Resized(size) => {
                if let Some(graphics_state) = &mut self.graphics_state {
                    graphics_state.resize(size.width, size.height)
                };
            }
            WindowEvent::MouseInput {
                state: btn_state,
                button,
                ..
            } => self
                .graphics_state
                .as_mut()
                .unwrap()
                .handle_mouse_button(button, btn_state.is_pressed()),
            WindowEvent::MouseWheel { delta, .. } => {
                self.graphics_state
                    .as_mut()
                    .unwrap()
                    .handle_mouse_scroll(&delta);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => self.graphics_state.as_mut().unwrap().handle_key(
                event_loop,
                code,
                key_state.is_pressed(),
            ),
            WindowEvent::RedrawRequested => {
                //TODO: check if this is actually necessary later
                if self.graphics_state.is_none() {
                    return;
                }

                let elapsed = self.last_iteration.elapsed();
                self.last_iteration = Instant::now();
                self.graphics_state
                    .as_mut()
                    .expect("GENERIC")
                    .update(elapsed);

                match self.graphics_state.as_mut().expect("fgfdsyxcfr").render(
                    self.world.to_be_rendered.iter().filter_map(|id| {
                        //TODO: .expect messages
                        //TODO: more efficient borrowing instead of only .as_ref / .as_mut

                        match self.world.entities.get(id).expect("").model {
                            ModelState::Visible(model_id) => Some(model_id),
                            _ => None,
                        }
                    }),
                ) {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = self.graphics_state.as_ref().expect("").window.inner_size();
                        self.graphics_state
                            .as_mut()
                            .expect("")
                            .resize(size.width, size.height);
                    }
                    Err(e) => {
                        log::error!("Unable to render {}", e);
                    }
                }
            }
            event => self.propagate_event(event),
        };
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        let state = if let Some(state) = &mut self.graphics_state {
            state
        } else {
            return;
        };
        match event {
            DeviceEvent::MouseMotion { delta: (dx, dy) } => {
                if state.mouse_pressed {
                    state.camera_controller.handle_mouse(dx, dy);
                }
            }
            _ => {}
        }
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        //the only possible error is a terminated clock and that's not a bad thing since we're terminating now
        let _ = self
            .termination_tx
            .as_ref()
            .expect("Always Some(_) while (and after) running the loop")
            .send(Terminate);
    }
}

//graphics state modifications
impl Game {
    ///Sets the clear color to the provided rgba color which is drawn to the whole screen before every render.
    /// Can be used as a provisoric one-color background
    pub fn with_color(mut self, r: f64, g: f64, b: f64, a: f64) -> Self {
        let config = self
            .graphics_state_config
            .as_mut()
            .expect("Should be Some(_) before running");
        config.color = wgpu::Color { r, g, b, a };
        self
    }

    pub fn with_model<P: Into<String>>(self, name: &'static str, path: P) -> Self {
        fn inner(mut game: Game, name: &'static str, path: String) -> Game {
            let config = game
                .graphics_state_config
                .as_mut()
                .expect("Should be Some(_) before running");
            config.models.insert(name, path);
            game
        }
        //as_ref() cosmetic here, but who cares...
        inner(self, name, path.into())
    }

    pub fn with_image<P: AsRef<Path>>(self, path: P, name: &'static str) -> Self {
        todo!();
        /*fn inner(mut game: Game, path: &Path, name: &'static str) -> Game {
            let config = game
                .graphics_state_config
                .as_mut()
                .expect("Should be Some(_) before running");
            config.assets.insert(name, Asset::Image(path.to_owned()));
            game
        }
        inner(self, path.as_ref(), name)*/
    }
}

//internal helper functions
impl Game {
    #[inline]
    fn propagate_event<T: 'static + Send>(&mut self, event: T) {
        self.events.add(event);
    }
}

use log::{error, warn};
use std::sync::{Arc, Mutex, RwLock, atomic};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
//use crate::custom_events::input::{SimpleKeyEvent, SimpleMouseKeyEvent};
use crate::aliases::{Commands, SendAnyMap};
use crate::commands::CommandHandle;
use crate::world::{ModelState, World};
use graphics_core::config::{Asset, StateConfig};
use winit::event_loop::{ControlFlow, EventLoopProxy};
use winit::window::WindowAttributes;
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};
