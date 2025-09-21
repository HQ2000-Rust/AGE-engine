use super::*;
use crate::commands::Command;
use crate::game::Game;
impl Command for SetPos {
    fn execute(&self, game: &mut Game) {
        game.world.move_entity(self.0, self.1);
    }
}

impl Command for SetPosRel {
    fn execute(&self, game: &mut Game) {
        game.world.move_entity_rel(self.0, self.1);
    }
}

impl Command for SetModel {
    fn execute(&self, game: &mut Game) {
        game.world.set_model(
            self.0,
            &game
                .graphics_state
                .as_ref()
                .expect("Should be Some(_) while running the game")
                .models,
            self.1,
        );
    }
}
