use cap::Cap;
use crates_index::*;
use rayon::iter::ParallelIterator;
use std::alloc;

#[global_allocator]
static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::max_value());

#[test]
fn mem_usage() {
    let index = Index::new_cargo_default().unwrap();

    let before = ALLOCATOR.allocated();
    // let all_crates: Vec<_> = index.crates().collect();
    let all_crates: Vec<_> = index.crates_parallel().map(|c| c.unwrap()).collect();
    let after = ALLOCATOR.allocated();
    let used = after - before;
    assert!(all_crates.len() > 76200);
    eprintln!(
        "used mem: {}B for {} crates, {}B per crate",
        used,
        all_crates.len(),
        used / all_crates.len()
    );
    assert!(used / all_crates.len() < 4700);
}
