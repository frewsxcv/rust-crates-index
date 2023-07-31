mod git;
mod names;
mod sparse_index;
mod error {
    #[test]
    fn error_is_send() {
        fn is_send<T: Send>() {}
        is_send::<crates_index::Error>();
    }
}

use crates_index::{Crate, Dependency, Version};

#[test]
fn sizes() {
    assert!(std::mem::size_of::<Version>() <= 152);
    assert!(std::mem::size_of::<Crate>() <= 16);
    assert!(std::mem::size_of::<Dependency>() <= 80);
}

#[test]
fn semver() {
    let c = Crate::from_slice(r#"{"vers":"1.0.0", "name":"test", "deps":[], "features":{}, "cksum":"1234567890123456789012345678901234567890123456789012345678901234", "yanked":false}
            {"vers":"1.2.0-alpha.1", "name":"test", "deps":[], "features":{}, "cksum":"1234567890123456789012345678901234567890123456789012345678901234", "yanked":false}
            {"vers":"1.0.1", "name":"test", "deps":[], "features":{}, "cksum":"1234567890123456789012345678901234567890123456789012345678901234", "yanked":false}"#.as_bytes()).unwrap();
    assert_eq!(c.most_recent_version().version(), "1.0.1");
    assert_eq!(c.highest_version().version(), "1.2.0-alpha.1");
    assert_eq!(c.highest_normal_version().unwrap().version(), "1.0.1");
}

#[test]
fn features2() {
    let c = Crate::from_slice(br#"{"vers":"1.0.0", "name":"test", "deps":[], "features":{"a":["one"], "b":["x"]},"features2":{"a":["two"], "c":["y"]}, "cksum":"1234567890123456789012345678901234567890123456789012345678901234"}"#).unwrap();
    let f2 = c.most_recent_version().features();

    assert_eq!(3, f2.len());
    assert_eq!(["one", "two"], &f2["a"][..]);
    assert_eq!(["x"], &f2["b"][..]);
    assert_eq!(["y"], &f2["c"][..]);
}

#[test]
fn rust_version() {
    let c = Crate::from_slice(br#"{"vers":"1.0.0", "name":"test", "deps":[], "features":{},"features2":{}, "cksum":"1234567890123456789012345678901234567890123456789012345678901234", "rust_version":"1.64.0"}"#).unwrap();
    assert_eq!(c.most_recent_version().rust_version(), Some("1.64.0"));
}
