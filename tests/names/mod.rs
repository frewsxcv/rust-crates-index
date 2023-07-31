use crates_index::Names;

#[test]
fn empty_string() {
    assert_eq!(Names::new("").unwrap().count(), 1);
}

#[test]
fn name_without_separators_yields_name() {
    assert_eq!(Names::new("serde").unwrap().count(), 1);
}

#[test]
fn permutation_count() {
    assert_eq!(Names::new("a-b").unwrap().count(), 2);
    assert_eq!(Names::new("a-b_c").unwrap().count(), 4);
    assert_eq!(Names::new("a_b_c").unwrap().count(), 4);
    assert_eq!(Names::new("a_b_c-d").unwrap().count(), 8);
}

#[test]
fn max_permutation_count_causes_error() {
    assert_eq!(
        Names::new("a-b-c-d-e-f-g-h-i-j-k-l-m-n-o-p")
            .expect("15 separators are fine")
            .count(),
        32768
    );
    assert!(
        Names::new("a-b-c-d-e-f-g-h-i-j-k-l-m-n-o-p-q-r").is_none(),
        "17 are not fine anymore"
    );
}

#[test]
fn permutations() {
    for (name, expected) in [
        ("a_b", &["a_b", "a-b"] as &[_]),
        ("a-b", &["a_b", "a-b"] as &[_]),
        ("a-b-c", &["a_b_c", "a-b_c", "a_b-c", "a-b-c"]),
    ] {
        let names: Vec<String> = Names::new(name).unwrap().collect();
        assert_eq!(&names, expected);
    }
}
