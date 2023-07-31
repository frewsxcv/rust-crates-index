use crates_index::Names;

#[test]
fn empty_string() {
    assert_eq!(Names::new("").count(), 1);
}

#[test]
fn name_without_separators_yields_name() {
    assert_eq!(Names::new("serde").count(), 1);
}

#[test]
fn permutation_count() {
    assert_eq!(Names::new("a-b").count(), 2);
    assert_eq!(Names::new("a-b_c").count(), 4);
    assert_eq!(Names::new("a_b_c").count(), 4);
    assert_eq!(Names::new("a_b_c-d").count(), 8);
}

#[test]
fn permutations() {
    for (name, expected) in [
        ("a_b", &["a_b", "a-b"] as &[_]),
        ("a-b", &["a_b", "a-b"] as &[_]),
        ("a-b-c", &["a_b_c", "a-b_c", "a_b-c", "a-b-c"]),
    ] {
        let names: Vec<String> = Names::new(name).collect();
        assert_eq!(&names, expected);
    }
}
