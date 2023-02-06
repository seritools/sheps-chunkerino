use std::marker::PhantomData;

super::ext!();

pub struct Parts<'a, T> {
    ptr: *const T,
    chunk_size_small: usize,
    n_small: usize,
    chunk_size_big: usize,
    n_big: usize,
    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T> Parts<'a, T> {
    pub fn new(slice: &'a [T], n_parts: usize) -> Self {
        let chunk_size = slice.len() / n_parts;
        let times_extra = slice.len() % n_parts;

        if slice.len() <= n_parts {
            Self {
                ptr: slice.as_ptr(),
                chunk_size_small: 1,
                n_small: slice.len(),
                chunk_size_big: 0,
                n_big: 0,
                _marker: PhantomData,
            }
        } else {
            Self {
                ptr: slice.as_ptr(),
                chunk_size_small: chunk_size,
                n_small: n_parts - times_extra,
                chunk_size_big: chunk_size + 1,
                n_big: times_extra,
                _marker: PhantomData,
            }
        }
    }
}

impl<'a, T> Iterator for Parts<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.n_small > 0 {
            self.n_small -= 1;
            let s = unsafe { std::slice::from_raw_parts(self.ptr, self.chunk_size_small) };
            self.ptr = unsafe { self.ptr.add(self.chunk_size_small) };
            Some(s)
        } else if self.n_big > 0 {
            self.n_big -= 1;
            let s = unsafe { std::slice::from_raw_parts(self.ptr, self.chunk_size_big) };
            self.ptr = unsafe { self.ptr.add(self.chunk_size_big) };
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
