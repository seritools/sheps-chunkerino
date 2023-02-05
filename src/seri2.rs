use std::marker::PhantomData;

super::ext!();

pub struct Parts<'a, T> {
    ptr: *const T,
    chunk_size1: usize,
    len1: usize,
    chunk_size2: usize,
    len2: usize,
    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T> Parts<'a, T> {
    pub fn new(slice: &'a [T], n_parts: usize) -> Self {
        let chunk_size = slice.len() / n_parts;
        let times_extra = slice.len() % n_parts;

        if slice.len() < n_parts {
            Self {
                ptr: slice.as_ptr(),
                chunk_size1: 1,
                len1: slice.len(),
                chunk_size2: 0,
                len2: 0,
                _marker: PhantomData,
            }
        } else if times_extra == 0 {
            Self {
                ptr: slice.as_ptr(),
                chunk_size1: chunk_size,
                len1: n_parts,
                chunk_size2: 0,
                len2: 0,
                _marker: PhantomData,
            }
        } else {
            Self {
                ptr: slice.as_ptr(),
                chunk_size1: chunk_size + 1,
                len1: times_extra,
                chunk_size2: chunk_size,
                len2: n_parts - times_extra,
                _marker: PhantomData,
            }
        }
    }
}

impl<'a, T> Iterator for Parts<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.len1 > 0 {
            self.len1 -= 1;
            let s = unsafe { std::slice::from_raw_parts(self.ptr, self.chunk_size1) };
            self.ptr = unsafe { self.ptr.add(self.chunk_size1) };
            Some(s)
        } else if self.len2 > 0 {
            self.len2 -= 1;
            let s = unsafe { std::slice::from_raw_parts(self.ptr, self.chunk_size2) };
            self.ptr = unsafe { self.ptr.add(self.chunk_size2) };
            Some(s)
        } else {
            None
        }
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
}
