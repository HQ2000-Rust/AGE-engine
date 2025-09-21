pub mod camera;
pub mod world;

use crate::game::Game;
use std::collections::VecDeque;

pub trait Command {
    //not efficient, will change that later - TODO
    fn execute(&self, game: &mut Game);
}

pub struct CommandHandle {
    commands: VecDeque<Box<dyn Command>>,
    queue: VecDeque<Box<dyn Command>>,
}

impl Default for CommandHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandHandle {
    pub fn new() -> Self {
        Self {
            commands: VecDeque::new(),
            queue: VecDeque::new(),
        }
    }
    #[inline]
    pub fn add<C: Command + 'static>(&mut self, command: C) {
        self.queue.push_back(Box::new(command));
    }
    pub fn add_vec<C: Command + 'static>(&mut self, commands: Vec<C>) {
        self.queue.append(
            &mut commands
                .into_iter()
                .map(|command| Box::new(command) as Box<dyn Command>)
                .collect(),
        );
    }
    //maybe split that later, depending on what I need
    fn update(&mut self, game: &mut Game) {
        self.commands.iter().for_each(|c| c.execute(game));
        self.commands = self.queue.drain(..).collect();
    }
}
