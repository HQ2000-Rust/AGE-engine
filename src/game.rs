use crate::custom_events::builtin::ExitEvent;
use crate::events::EventHandle;
use crate::scheduler::Scheduler;
use anymap::AnyMap;
use graphics_core::state::State;

pub struct Game {
    resources: AnyMap,
    scheduler: Scheduler,
    events: EventHandle,
    //only existent while setup
    graphics_state_config: Option<StateConfig>,
    graphics_state: Option<State>,
    //make it customisable later
    window_attributes: WindowAttributes,
    //TODO!: commands scheduler
    commands: Commands,
    //?
}

impl Default for Game {
    fn default() -> Self {
        Self {
            resources: AnyMap::new(),
            scheduler: Scheduler::new_empty(),
            events: EventHandle::new(),
            graphics_state: None,
            window_attributes: WindowAttributes::default(),
            commands: Commands::new(),
            graphics_state_config: Some(StateConfig::default()),
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_once(
        mut self,
        once: impl FnMut(&mut Commands, &mut AnyMap, &mut EventHandle) + 'static,
    ) -> Self {
        self.scheduler.add_once(Box::new(once));
        self
    }

    pub fn add_update(
        mut self,
        update: impl FnMut(&mut Commands, &mut AnyMap, &mut EventHandle) + 'static,
    ) -> Self {
        self.scheduler.add_update(Box::new(update));
        self
    }

    #[inline]
    pub fn prepare(&mut self) {
        self.scheduler
            .setup(&mut self.commands, &mut self.resources, &mut self.events);
        self.events.update();
    }

    fn exit(&mut self, event_loop: ActiveEventLoop) {}

    #[inline]
    pub fn run(mut self) {
        env_logger::init();
        warn!("Starting event loop");
        //TODO!: nicer error handling -> (inline) function
        let event_loop = EventLoop::with_user_event().build();
        if let Err(e) = event_loop {
            let message = format!("Fatal error while building the event loop: {}", e);
            error!("{}", message);
            panic!("{}", message);
        }
        self.prepare();
        if let Err(e) = event_loop
            .expect("Would have panicked if there was an error with the event loop")
            .run_app(&mut self)
        {
            let message = format!("Fatal error while running the event loop: {}", e);
            error!("{}", message);
            panic!("{}", message);
        }
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
}

//internal helper functions
impl Game {
    #[inline]
    fn propagate_event<T: 'static>(&mut self, event: T) {
        self.events.add(event);
    }
}

use log::{error, warn};
use std::sync::Arc;
//use crate::custom_events::input::{SimpleKeyEvent, SimpleMouseKeyEvent};
use crate::aliases::Commands;
use graphics_core::config::StateConfig;
use winit::window::WindowAttributes;
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

impl ApplicationHandler<Game> for Game {
    /*fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        todo!()
    }*/

    //TODO!: make sure initialisation only happens once, resume() doesn't guarantee that!!
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
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
            .expect("State creation needs to work"),
        );
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: Game) {
        *self = event;
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        self.scheduler
            .update(&mut self.commands, &mut self.resources, &mut self.events);
        self.events.update();

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            //I should refactor that later
            WindowEvent::Resized(size) => {
                if let Some(graphics_state) = &mut self.graphics_state {
                    graphics_state.resize(size.width, size.height)
                };
            }
            WindowEvent::RedrawRequested => {
                if let Some(graphics_state) = &mut self.graphics_state {
                    graphics_state.update();
                    match graphics_state.render() {
                        Ok(_) => {}
                        // Reconfigure the surface if it's lost or outdated
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            let size = graphics_state.window.inner_size();
                            graphics_state.resize(size.width, size.height);
                        }
                        Err(e) => {
                            log::error!("Unable to render {}", e);
                        }
                    }
                }
            }
            event => self.propagate_event(event),
        };
    }
    fn memory_warning(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.exit();
        panic!("Received a memory warning, currently a fatal error");
        todo!();
    }
}
