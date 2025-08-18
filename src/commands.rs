use graphics_core::state::State;
use std::collections::VecDeque;

//T because I can't use the graphical State here, maybe this even pays off later
pub trait Command {
    fn execute(&self, state: &mut State);
}

pub struct CommandHandle {
    commands: VecDeque<Box<dyn Command>>,
    queue: VecDeque<Box<dyn Command>>,
}

impl CommandHandle {
    pub fn new() -> Self {
        Self {
            commands: VecDeque::new(),
            queue: VecDeque::new(),
        }
    }
    pub fn add<C: Command + 'static>(&mut self, command: C) {
        self.queue.push_back(Box::new(command));
    }
    //maybe split that later, depending on what I need
    fn update(&mut self, state: &mut State) {
        self.commands.iter().for_each(|c| c.execute(state));
        self.commands = self.queue.drain(..).collect();
    }
}
