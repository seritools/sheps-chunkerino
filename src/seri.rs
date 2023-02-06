super::ext!();

pub struct Parts<'a, T> {
    slice: &'a [T],
    chunk_size: usize,
    times_extra: usize,
}

impl<'a, T> Parts<'a, T> {
    #[inline]
    pub fn new(slice: &'a [T], n_parts: usize) -> Self {
        if slice.len() <= n_parts {
            return Self {
                slice,
                chunk_size: 1,
                times_extra: 0,
            };
        }

        let chunk_size = slice.len() / n_parts;
        let times_extra = slice.len() % n_parts;

        Self {
            slice,
            chunk_size,
            times_extra,
        }
    }
}

impl<'a, T> Iterator for Parts<'a, T> {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            slice,
            mut chunk_size,
            times_extra,
        } = *self;

        if slice.is_empty() {
            return None;
        }

        if times_extra > 0 {
            chunk_size += 1;
            self.times_extra -= 1;
        }

        let (head, tail) = unsafe { slice.split_at_unchecked(chunk_size) };
        self.slice = tail;

        Some(head)
    }
}

impl DoubleEndedIterator for Parts<'_, u8> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let Self {
            slice,
            mut chunk_size,
            times_extra,
        } = *self;

        if slice.is_empty() {
            return None;
        }

        if times_extra > 0 {
            chunk_size += 1;
            self.times_extra -= 1;
        }

        let (head, tail) = slice.split_at(slice.len() - chunk_size);
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

    // #[test]
    // fn rev_test() {
    //     let a = [0, 1, 2, 3, 4];

    //     let v: Vec<_> = a.parts(2).rev().collect();
    //     assert_eq!(v, vec![&[2, 3, 4][..], &[0, 1][..]]);
    // }
}
