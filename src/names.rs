/// Iterator over all possible permutations of `-_` in crate names
#[derive(Clone)]
pub struct Names {
    count: u32,
    max_count: u32,
    chars: Vec<char>,
    separator_indexes: Vec<usize>,
}

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
