extern crate core;

mod aliases;
mod camera;
mod commands;
mod custom_events;
mod events;
mod game;
mod game_fn;
#[cfg_attr(feature="python",pyo3::pymodule(name = "age_engine"))]
pub mod prelude;
mod scheduler;
mod world;
