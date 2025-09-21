use super::*;
use crate::commands::Command;
use crate::game::Game;
use cgmath::prelude::*;

//TODO
/*impl Command for SetSpeed {
    fn execute(&self, game: &mut Game) {
        let mut camera=game.graphics_state.as_mut().expect("Should be Some(_) now, Game is running").camera;
        camera.speed = self.0;
    }
}

//TODO!: remove unchecked subs/muls/adds/divs - or not :-)
//the .0 fields correspond to the velocity

impl Command for Up {
    fn execute(&self, game: &mut Game) {
        let mut camera=game.graphics_state.as_mut().expect("Should be Some(_) now, Game is running").camera;
        todo!();
    }
}

impl Command for UpWithSpeed {
    fn execute(&self, game: &mut Game) {
        let mut camera=game.graphics_state.as_mut().expect("Should be Some(_) now, Game is running").camera;
        todo!()
    }
}

impl Command for Down {
    fn execute(&self, game: &mut Game) {
        let mut camera=game.graphics_state.as_mut().expect("Should be Some(_) now, Game is running").camera;
        todo!()
    }
}

impl Command for DownWithSpeed {
    fn execute(&self, game: &mut Game) {
        let mut camera=&mut game.graphics_state.as_mut().expect("Should be Some(_) now, Game is running").camera;
        todo!()
    }
}

impl Command for Left {
    fn execute(&self, game: &mut Game) {
        let projection=&mut game.graphics_state.as_mut().expect("Should be Some(_) now, Game is running").;
        LeftWithSpeed(projection.speed).execute(game);
    }
}

impl Command for LeftWithSpeed {
    fn execute(&self, game: &mut Game) {
        let mut camera=game.graphics_state.as_mut().expect("Should be Some(_) now, Game is running").camera;
    }
}

impl Command for Right {
    fn execute(&self, game: &mut Game) {
        let mut camera=game.graphics_state.as_mut().expect("Should be Some(_) now, Game is running").camera;
        RightWithSpeed(camera.speed).execute(game);
    }
}

impl Command for RightWithSpeed {
    fn execute(&self, game: &mut Game) {
        let mut camera=game.graphics_state.as_mut().expect("Should be Some(_) now, Game is running").camera;
    }
}

impl Command for Forward {
    fn execute(&self, game: &mut Game) {
        let mut camera=game.graphics_state.as_mut().expect("Should be Some(_) now, Game is running").camera;
        ForwardWithSpeed(camera.speed).execute(game);
    }
}

impl Command for ForwardWithSpeed {
    fn execute(&self, game: &mut Game) {

    }
}

impl Command for Backward {
    fn execute(&self, game: &mut Game) {
        let mut camera=game.graphics_state.as_mut().expect("Should be Some(_) now, Game is running").camera;
        BackwardWithSpeed(camera.speed).execute(game);
    }
}

impl Command for BackwardWithSpeed {
    fn execute(&self, game: &mut Game) {
        let mut camera=game.graphics_state.as_mut().expect("Should be Some(_) now, Game is running").camera;

    }
}
*/
