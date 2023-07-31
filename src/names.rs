/// Iterator over all possible permutations of `-_` in crate names
#[derive(Clone)]
pub struct Names {
    count: u16,
    max_count: u16,
    chars: Vec<char>,
    separator_indexes: [usize; 17],
    separator_count: usize,
}

impl Names {
    /// Creates a new iterator over all permutations of `-` and `_` of the given `name`,
    /// or `None` if there are more than 15 `-` or `_` characters.
    pub fn new(name: &str) -> Option<Names> {
        let mut separator_indexes = [0; 17];

        let mut separator_count = 0;
        let chars: Vec<char> = name
            .chars()
            .enumerate()
            .map(|(index, char)| {
                if char == '-' || char == '_' {
                    separator_indexes[separator_count] = index;
                    separator_count += 1;
                    '-'
                } else {
                    char
                }
            })
            .collect();

        Some(Names {
            count: 0,
            max_count: 2u16.checked_pow(separator_count.try_into().ok()?)?,
            chars,
            separator_indexes,
            separator_count,
        })
    }
}

impl Iterator for Names {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.max_count {
            return None;
        }

        for (index, string_index) in self.separator_indexes[..self.separator_count].iter().enumerate() {
            let char = if self.count & (1 << index) == 0 { '_' } else { '-' };
            *self.chars.get_mut(*string_index).unwrap() = char;
        }

        self.count += 1;
        Some(self.chars.iter().collect())
    }
}
