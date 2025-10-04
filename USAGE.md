> -**sadly, uploading this to crates.io would cause heavy namespace conflicts...** (it's a workspace)
> - for the SoM reviewers: please just pretend that this is this [README section](https://github.com/HQ2000-Rust/AGE_engine?tab=readme-ov-file#usage) or the [release](https://github.com/HQ2000-Rust/AGE_engine/releases/tag/v0.1.0), I forgot it when I shipped...

download the [release](https://github.com/HQ2000-Rust/AGE_engine/releases/tag/v0.1.1) and refer to it in your Cargo.toml like this:

```toml
[dependencies]
age_engine={ path="path/to/age_engine" }
```
