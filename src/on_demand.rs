super::ext!();

pub struct Parts<'a, T> {
    slice: &'a [T],
    n_parts: usize,
}

impl<'a, T> Parts<'a, T> {
    pub fn new(slice: &'a [T], n_parts: usize) -> Self {
        Self { slice, n_parts }
    }
}

impl<'a, T> Iterator for Parts<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        let Self { slice, n_parts } = *self;

        if n_parts == 0 {
            return None;
        }

        let mut chunk_size = slice.len() / n_parts;
        let leftover = slice.len() % n_parts;

        if leftover != 0 {
            chunk_size += 1;
        }

        // Or on nightly
        // let chunk_size = slice.len().div_ceil(n_parts);

        let (head, tail) = unsafe { slice.split_at_unchecked(chunk_size) };
        self.slice = tail;
        self.n_parts -= 1;

        Some(head)
    }
}
