#[cfg(all(feature = "parallel", feature = "git"))]
mod mem {
    use cap::Cap;
    use std::alloc;
    use std::time::Instant;
    use bytesize::ByteSize;

    #[global_allocator]
    static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::max_value());

    #[test]
    #[cfg_attr(debug_assertions, ignore = "too slow when running in debug mode")]
    fn usage() {
        use crates_index::GitIndex;
        use rayon::iter::ParallelIterator;

        let index = GitIndex::new_cargo_default().unwrap();

        let before = ALLOCATOR.allocated();
        // let all_crates: Vec<_> = index.crates().collect();
        let start = Instant::now();
        let all_crates: Vec<_> = index.crates_parallel().map(|c| c.unwrap()).collect();
        let after = ALLOCATOR.allocated();
        let used = after - before;
        assert!(all_crates.len() > 89000);
        let elapsed = start.elapsed().as_secs_f32();
        let per_crate = used / all_crates.len();
        eprintln!(
            "used mem: {}B for {} crates, {}B per crate, took {elapsed:.02}s [total-mem: {total}, peak-mem: {peak}]",
            ByteSize(used as u64),
            all_crates.len(),
            per_crate,
            total = ByteSize(ALLOCATOR.total_allocated() as u64),
            peak = ByteSize(ALLOCATOR.max_allocated() as u64),
        );
        assert!(per_crate < 6300, "per crate limit {per_crate}B should remain below memory limit");
    }
}
