use crate::custom_events::builtin::ExitEvent;
use crate::events::EventHandle;
use crate::scheduler::Scheduler;
use anymap::AnyMap;
use graphics_core::state::State;

pub struct Game {
    resources: AnyMap,
    scheduler: Scheduler,
    events: EventHandle,
    graphics_state: Option<State>,
    //make it customisable later
    window_attributes: WindowAttributes,
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
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_once(mut self, once: impl FnMut(&mut AnyMap, &mut EventHandle) + 'static) -> Self {
        self.scheduler.add_once(Box::new(once));
        self
    }

    pub fn add_update(
        mut self,
        update: impl FnMut(&mut AnyMap, &mut EventHandle) + 'static,
    ) -> Self {
        self.scheduler.add_update(Box::new(update));
        self
    }

    #[inline]
    fn propagate_event<T: 'static>(&mut self, event: T) {
        self.events.add(event);
    }

    #[inline]
    pub fn prepare(&mut self) {
        self.scheduler.setup(&mut self.resources, &mut self.events);
        self.events.update();

        /*while !self.events.contains(&ExitEvent) {
            self.scheduler
                .update(&mut self.resources, &mut self.events);
            self.events.update();
        }*/
    }

    pub fn exit(&mut self, event_loop: ActiveEventLoop) {}

    //let it return () later (TODO!)
    pub fn run(mut self) -> anyhow::Result<()> {
        env_logger::init();
        let event_loop = EventLoop::with_user_event().build()?;
        self.prepare();
        event_loop.run_app(&mut self)?;
        Ok(())
    }
}

use std::{iter, sync::Arc};

//use crate::custom_events::input::{SimpleKeyEvent, SimpleMouseKeyEvent};
use winit::window::WindowAttributes;
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

impl ApplicationHandler<Game> for Game {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(self.window_attributes.clone())
                .expect(
                    "Winit needs to be able to create a window, otherwise everything won't work",
                ),
        );

        self.graphics_state =
            Some(pollster::block_on(State::new(window)).expect("State creation needs to work"));
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
        self.scheduler.update(&mut self.resources, &mut self.events);
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
            },
            event =>  self.propagate_event(event),
        };
    }
}
