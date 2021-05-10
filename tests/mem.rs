use cap::Cap;
use crates_index::*;
use std::alloc;

#[global_allocator]
static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::max_value());

#[test]
fn mem_usage() {
    let index = Index::new_cargo_default();
    assert!(index.exists());

    let before = ALLOCATOR.allocated();
    let all_crates: Vec<_> = index.crates().collect();
    let after = ALLOCATOR.allocated();
    let used = after - before;
    eprintln!("used mem: {}B for {} crates", used, all_crates.len());
    assert!(used / all_crates.len() < 4100);
}
