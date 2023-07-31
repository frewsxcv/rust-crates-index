use crates_index::Names;

fn data_count(names: Names) -> usize {
    names.collect::<Vec<String>>().len()
}

#[test]
fn empty_string() {
    assert_eq!(data_count(Names::new("").unwrap()), 1);
}

#[test]
fn name_without_separators_yields_name() {
    assert_eq!(data_count(Names::new("serde").unwrap()), 1);
}

#[test]
fn permutation_count() {
    assert_eq!(Names::new("a-b").unwrap().count(), 2);
    assert_eq!(Names::new("a-b_c").unwrap().count(), 4);
    assert_eq!(Names::new("a_b_c").unwrap().count(), 4);
    assert_eq!(Names::new("a_b_c-d").unwrap().count(), 8);
}

#[test]
fn permutation_data_count() {
    assert_eq!(data_count(Names::new("a-b").unwrap()), 2);
    assert_eq!(data_count(Names::new("a-b_c").unwrap()), 4);
    assert_eq!(data_count(Names::new("a_b_c").unwrap()), 4);
    assert_eq!(data_count(Names::new("a_b_c-d").unwrap()), 8);
}

#[test]
fn max_permutation_count_causes_error() {
    assert_eq!(
        data_count(Names::new("a-b-c-d-e-f-g-h-i-j-k-l-m-n-o-p").expect("15 separators are fine")),
        32768
    );
    assert!(
        Names::new("a-b-c-d-e-f-g-h-i-j-k-l-m-n-o-p-q-r").is_none(),
        "16 are not fine anymore"
    );
}

#[test]
fn permutations() {
    for (name, expected) in [
        ("parking_lot", &["parking_lot", "parking-lot"] as &[_]), // the input name is always the first one returned.
        ("a_b", &["a_b", "a-b"]),
        ("a-b", &["a-b", "a_b"]),
        ("a-b-c", &["a-b-c", "a_b-c", "a-b_c", "a_b_c"]),
        (
            "a-b-c-d",
            &[
                "a-b-c-d", "a_b-c-d", "a-b_c-d", "a_b_c-d", "a-b-c_d", "a_b-c_d", "a-b_c_d", "a_b_c_d",
            ],
        ),
        (
            "a_b_c_d",
            &[
                "a_b_c_d", "a-b-c-d", "a_b-c-d", "a-b_c-d", "a_b_c-d", "a-b-c_d", "a_b-c_d", "a-b_c_d",
            ],
        ),
    ] {
        let names: Vec<String> = Names::new(name).unwrap().collect();
        assert_eq!(&names, expected);
    }
}
