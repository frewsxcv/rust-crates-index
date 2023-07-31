use crates_index::Names;
use itertools::Itertools;

#[test]
fn empty_string() {
    assert_eq!(Names::new("").dedup().count(), 1);
}

#[test]
fn string_with_out_separators() {
    assert_eq!(Names::new("serde").dedup().count(), 1);
}

#[test]
fn string_with_separators() {
    assert_eq!(Names::new("serde-test").dedup().count(), 2);
    assert_eq!(Names::new("serde-test_2").dedup().count(), 4);
    assert_eq!(Names::new("serde_test_2").dedup().count(), 4);
    assert_eq!(Names::new("serde_test_2-test").dedup().count(), 8);
}

#[test]
fn correct_strings() {
    let names = Names::new("serde-test-2");

    assert!(names.clone().contains(&"serde-test-2".to_string()));
    assert!(names.clone().contains(&"serde-test_2".to_string()));
    assert!(names.clone().contains(&"serde_test_2".to_string()));
    assert!(names.clone().contains(&"serde_test-2".to_string()));
}
