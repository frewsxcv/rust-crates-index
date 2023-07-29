use gix::bstr::ByteSlice;
use super::*;

#[test]
#[cfg_attr(debug_assertions, ignore = "too slow in debug mode")]
fn parse_all_blobs() {
    std::thread::scope(|scope| {
        let (tx, rx) = std::sync::mpsc::channel();
        let blobs = scope.spawn(move || {
            let index = shared_index();
            for c in index.crates_blobs().unwrap() {
                tx.send(c).unwrap();
            }
        });
        let parse = scope.spawn(move || {
            let mut found_gcc_crate = false;
            let mut ctx = DedupeContext::new();
            for c in rx {
                match c.parse(&mut ctx) {
                    Ok(c) => {
                        if c.name() == "gcc" {
                            found_gcc_crate = true;
                        }
                    }
                    Err(e) => panic!("can't parse :( {:?}: {e}", c.0.as_bstr()),
                }
            }
            assert!(found_gcc_crate);
        });
        parse.join().unwrap();
        blobs.join().unwrap();
    });
}

fn shared_index() -> GitIndex {
    let index_path = "tests/fixtures/git-registry";
    if is_ci::cached() {
        GitIndex::new_cargo_default()
            .expect("CI has just cloned this index and its ours and valid")
    } else {
        GitIndex::with_path(index_path, URL).expect("clone works and there is no racing")
    }
}
