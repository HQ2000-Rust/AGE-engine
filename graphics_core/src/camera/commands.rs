use super::Camera;
use cgmath::prelude::*;
pub trait CameraCommand {
    fn apply(&self, camera: &mut Camera);
}

//TODO!: remove unchecked subs/muls/adds/divs

//the .0 fields correspond to the velocity
pub struct Up(f32);
impl CameraCommand for Up {
    fn apply(&self, camera: &mut Camera) {
        todo!();
    }
}
pub struct Down(f32);
impl CameraCommand for Down {
    fn apply(&self, camera: &mut Camera) {
        todo!()
    }
}
pub struct Left(f32);
impl CameraCommand for Left {
    fn apply(&self, camera: &mut Camera) {
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();
        let forward_norm = forward.normalize();
        let right = forward_norm.cross(camera.up);
        camera.eye = camera.target - (forward - right * self.0).normalize() * forward_mag;
    }
}
pub struct Right(f32);
impl CameraCommand for Right {
    fn apply(&self, camera: &mut Camera) {
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();
        let forward_norm = forward.normalize();
        let right = forward_norm.cross(camera.up);
        camera.eye = camera.target - (forward + right * self.0).normalize() * forward_mag;
    }
}
pub struct Forward(f32);
impl CameraCommand for Forward {
    fn apply(&self, camera: &mut Camera) {
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();
        let forward_norm = forward.normalize();
        //supposed to prevent glitching (-> sotrh)
        if forward_mag > self.0 {
            camera.eye += forward_norm * self.0;
        }
    }
}
pub struct Backward(f32);
impl CameraCommand for Backward {
    fn apply(&self, camera: &mut Camera) {
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        camera.eye -= forward_norm * self.0;
    }
}
