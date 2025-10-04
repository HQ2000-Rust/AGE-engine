# AGE_engine
AGE, short for "Another Game Engine", is supposed to be a simple toy game engine **library** written in Rust with basic support for things like 2d & 3d graphics, assets and so on. Making a project using it should not take much boilerplate, be straightforward and also beginner-friendly. It is mostly working, except for the graphics.
## Usage
> **sadly, uploading this to crates.io would cause heavy namespace conflicts...** (it's a workspace)

download the [release](https://github.com/HQ2000-Rust/AGE_engine/releases/tag/v0.1.1) and refer to it in your Cargo.toml like this:

```toml
[dependencies]
age_engine={ path="path/to/age_engine" }
```
## Example code
Hello World
```rust
use age_engine::prelude::*;

fn main() {
    Game::new().add_once(once).run();
}

fn once(commands: &mut Commands, resources: &mut SendAnyMap, event_handle: &mut EventHandle) {
    println!("Hello World!");
}
**Im´t is recommended to try it out by yourself as there's so much more to do than just this! (I just didn't have enough time for more examples)**
```
## Inspiration
Even though this project isn't finished by far, I already have some sources that do and will influence this project:
- The awesome Bevy engine: **Game Builder Pattern** (it's probably not unique to the engine, but it inspired me, so I utilize it too), **Systems** (My engine isn't an ECS, but you can add startup and update functions with three parameters), **Resources** (prob also not really unique, but I do know it from there, also, the AnyMap isn't really inspired there, I found it somewhere else and since it's so convenient I use it)
- [Sotrh](https://github.com/sotrh)'s awesome wgpu tutorial (I build on top of it to have a working basis, but that may change)
> So, as you see, that's a lot of inspiration. I could've done things differently, but I don't see the point in deliberatly trying to do it in another (probably way more difficult way), also I do have a time restriction (end of the month)
## Goals
> [PROJECT_GOALS.md](https://github.com/HQ2000-Rust/AGE_engine/blob/main/PROJECT_GOALS.md)
