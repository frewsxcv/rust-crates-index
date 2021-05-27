# rust-crates-index

[![crates-index on Crates.io](https://meritbadge.herokuapp.com/crates-index)](https://crates.io/crates/crates-index)

Library for retrieving and interacting with the [crates.io registry git-based index](https://github.com/rust-lang/crates.io-index).

[Documentation](https://docs.rs/crates-index/)

## Example

```rust
let index = crates_index::Index::new("index_checkout");
if !index.exists() {
    index.retrieve().expect("Could not retrieve crates.io index");
}
for crate_releases in index.crates() {
    let _ = crate_releases.latest_version(); // any version most recently published
    let crate_version = crate_releases.highest_version(); // max version by semver
    println!("crate name: {}", crate_version.name());
    println!("crate version: {}", crate_version.version());
}
```

## Similar crates

- [`crates_io_api`](https://github.com/theduke/crates_io_api)

## License

Licensed under version 2 of the Apache License
