/// An iterator over all possible permutations of hyphens (`-`) and underscores (`_`) of a crate name.
///
/// The sequence yields the input name first, then an all-hyphens variant of it followed by an
/// all-underscores variant to maximize the chance of finding a match. Then follow all remaining permutations.
///
/// For instance, the name `parking_lot` is turned into the sequence `parking_lot` and `parking-lot`, while
/// `serde-yaml` is turned into `serde-yaml` and `serde_yaml`.
/// Finally, `a-b_c`  is returned as `a-b_c`, `a-b-c`, `a_b_c`, `a_b-c`.
#[derive(Clone)]
pub struct Names {
    count: Option<u16>,
    initial: String,
    max_count: u16,
    current: String,
    separator_indexes: [usize; 17],
    separator_count: usize,
}

impl Names {
    /// Creates a new iterator over all permutations of `-` and `_` of the given `name`,
    /// or `None` if there are more than 15 `-` or `_` characters.
    pub fn new(name: impl Into<String>) -> Option<Names> {
        let mut separator_indexes = [0; 17];
        let mut separator_count = 0;

        let name = name.into();
        let current: String = name
            .chars()
            .enumerate()
            .map(|(index, char)| {
                if char == '-' || char == '_' {
                    separator_indexes[separator_count] = index;
                    separator_count += 1;
                    '_'
                } else {
                    char
                }
            })
            .collect();

        Some(Names {
            count: None,
            initial: name,
            max_count: 2u16.checked_pow(separator_count.try_into().ok()?)?,
            current,
            separator_indexes,
            separator_count,
        })
    }
}

impl Iterator for Names {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.count.as_mut() {
            None => {
                self.count = Some(0);
                self.initial.clone().into()
            }
            Some(count) => {
                for _round in 0..2 {
                    if *count == self.max_count {
                        return None;
                    }

                    //map the count so the first value is the last one (all "-"), the second one is the first one (all "_")...
                    let used_count = *count as isize - 1 + self.max_count as isize;
                    for (sep_index, char_index) in self.separator_indexes[..self.separator_count].iter().enumerate() {
                        let char = if used_count & (1 << sep_index) == 0 { b'_' } else { b'-' };
                        // SAFETY: We validated that `char_index` is a valid UTF-8 codepoint
                        #[allow(unsafe_code)]
                        unsafe {
                            self.current.as_bytes_mut()[*char_index] = char;
                        }
                    }

                    *count += 1;
                    if self.current != self.initial {
                        break;
                    }
                }
                Some(self.current.clone())
            }
        }
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.max_count as usize
    }
}
