# crates-index

[![crates-index on Crates.io](https://img.shields.io/crates/v/crates-index.svg)](https://crates.io/crates/crates-index)

Library for retrieving and interacting with the [crates.io registry index](https://doc.rust-lang.org/cargo/reference/registry-index.html) using either the `git` or `sparse` protocol.

The index contains metadata for all Rust libraries and programs published on crates.io: their versions, dependencies, and feature flags.

[Documentation](https://docs.rs/crates-index/)

## Example

```rust
let index = crates_index::Index::new_cargo_default()?;

for crate_releases in index.crates() {
    let _ = crate_releases.most_recent_version(); // newest version
    let crate_version = crate_releases.highest_version(); // max version by semver
    println!("crate name: {}", crate_version.name());
    println!("crate version: {}", crate_version.version());
}
```

## Changelog

Please find the changelog in [CHANGELOG.md](https://github.com/frewsxcv/rust-crates-index/blob/master/CHANGELOG.md).

## Similar crates

- [`tame-index`](https://github.com/EmbarkStudios/tame-index) - a hard fork with many improvements and advantages
- [`crates_io_api`](https://github.com/theduke/crates_io_api) - a way to talk to the HTTP API of crates.io

## License

Licensed under version 2 of the Apache License
