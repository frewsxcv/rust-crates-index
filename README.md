# rust-crates-index

[![crates-index on Crates.io](https://meritbadge.herokuapp.com/crates-index)](https://crates.io/crates/crates-index)

Library for retrieving and interacting with the [crates.io index](https://github.com/rust-lang/crates.io-index)

[Documentation](https://docs.rs/crates-index/)

Much of this code was extracted from [github.com/huonw/crates.io-graph](https://github.com/huonw/crates.io-graph)

## Examples

```rust
let index = crates_index::Index::new("_index".into());
if !index.exists() {
    index.retrieve().expect("Could not retrieve crates.io index");
}
for crate_ in index.crates() {
    let most_recent = crate_.latest_version();
    println!("crate name: {}", most_recent.name());
    println!("crate version: {}", latest_version.version());
}
```

## Similar crates

- [`crates_io_api`](https://github.com/theduke/crates_io_api)

## License

Licensed under version 2 of the Apache License
