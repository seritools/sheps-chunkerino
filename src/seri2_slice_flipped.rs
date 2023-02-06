super::ext!();

pub struct Parts<'a, T> {
    slice: &'a [T],
    chunk_size_small: usize,
    n_small: usize,
    chunk_size_big: usize,
    n_big: usize,
}

impl<'a, T> Parts<'a, T> {
    #[inline]
    pub fn new(slice: &'a [T], n_parts: usize) -> Self {
        if slice.len() <= n_parts {
            // optimize trivial case
            Self {
                slice,
                chunk_size_small: 1,
                n_small: slice.len(),
                chunk_size_big: 0,
                n_big: 0,
            }
        } else {
            let chunk_size = slice.len() / n_parts;
            let bigger_chunk_count = slice.len() % n_parts;

            Self {
                slice,
                chunk_size_small: chunk_size,
                n_small: n_parts - bigger_chunk_count,
                chunk_size_big: chunk_size + 1,
                n_big: bigger_chunk_count,
            }
        }
    }
}

impl<'a, T> Iterator for Parts<'a, T> {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (head, tail) = if self.n_small > 0 {
            self.n_small -= 1;
            unsafe { self.slice.split_at_unchecked(self.chunk_size_small) }
        } else if self.n_big > 0 {
            self.n_big -= 1;
            unsafe { self.slice.split_at_unchecked(self.chunk_size_big) }
        } else {
            return None;
        };

        self.slice = tail;

        Some(head)
    }
}

impl<'a, T> DoubleEndedIterator for Parts<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let (head, tail) = if self.n_small > 0 {
            self.n_small -= 1;
            unsafe {
                self.slice
                    .split_at_unchecked(self.slice.len() - self.chunk_size_small)
            }
        } else if self.n_big > 0 {
            self.n_big -= 1;
            unsafe {
                self.slice
                    .split_at_unchecked(self.slice.len() - self.chunk_size_big)
            }
        } else {
            return None;
        };

        self.slice = head;

        Some(tail)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn cow_does_it() {
        // Does not (currently) have empty values.
        let a = [1];
        assert_eq!(1, a.parts(100).count());

        // Check our properties across a range of values
        for item_count in 0..=100 {
            let items = vec![(); item_count];

            for requested_parts in 1..(items.len() * 2) {
                let actual_parts = items.parts(requested_parts).count();
                assert!(actual_parts <= requested_parts);

                let unique_chunk_lens = items
                    .parts(requested_parts)
                    .map(|c| c.len())
                    // .inspect(|x| eprintln!("{x}"))
                    .collect::<BTreeSet<_>>();
                assert!(unique_chunk_lens.len() <= 2);
                // eprintln!("");

                if !unique_chunk_lens.is_empty() {
                    let mut unique_values = unique_chunk_lens.into_iter();
                    let short_len = unique_values.next().unwrap();
                    if let Some(long_len) = unique_values.next() {
                        assert_eq!(long_len, short_len + 1);
                    }
                }
            }
        }
        // panic!()
    }

    #[test]
    fn rev_test() {
        let a = [0, 1, 2, 3, 4];

        let v: Vec<_> = a.parts(2).rev().collect();
        assert_eq!(v, vec![&[3, 4][..], &[0, 1, 2][..]]);
    }
}
