use crate::Names;

impl Names {
    /// creates a new Iterator based on the given name
    pub fn new(name: &str) -> Names {
        let mut separator_indexes = vec![];

        let chars: Vec<char> = name
            .chars()
            .enumerate()
            .map(|(index, char)| {
                if char == '-' || char == '_' {
                    separator_indexes.push(index);
                    return '-';
                }

                char
            })
            .collect();

        Names {
            count: 0,
            max_count: 2u32.pow(separator_indexes.len() as u32),
            chars,
            separator_indexes,
        }
    }
}

impl Iterator for Names {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.max_count {
            return None;
        }

        for (index, string_index) in self.separator_indexes.iter().enumerate() {
            let char = if self.count & (1 << index) == 0 { '_' } else { '-' };

            *self.chars.get_mut(*string_index).unwrap() = char;
        }

        self.count += 1;

        Some(self.chars.iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::Names;
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
}
