> **sadly, uploading this to crates.io would cause heavy namespace conflicts...** (it's a workspace)

download the [release](https://github.com/HQ2000-Rust/AGE_engine/releases/tag/v0.1.0) and refer to it in your Cargo.toml like this:

```toml
[dependencies]
age_engine={ path="path/to/age_engine" }
```
