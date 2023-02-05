super::ext!();

pub struct Parts<'a, T> {
    slice: &'a [T],
    n_head: usize,
    n_tail: usize,
    head_len: usize,
    tail_len: usize,
}

impl<'a, T> Parts<'a, T> {
    // Not happy about zero
    pub fn new(slice: &'a [T], n_parts: usize) -> Self {
        // let tail_len = slice.len() / n_parts;
        // let remainder = slice.len() % n_parts;
        // let head_len = tail_len + 1;
        // let n_head = remainder;
        // let n_tail = n_parts - n_head;

        let len = slice.len();
        let tail_len = len / n_parts;
        let head_len = tail_len + 1;
        let n_head = len - (tail_len * n_parts);
        let n_tail = n_parts - n_head;

        Self {
            slice,
            n_head,
            n_tail,
            head_len,
            tail_len,
        }
    }
}

impl<'a, T> Iterator for Parts<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            slice,
            n_head,
            n_tail,
            head_len,
            tail_len,
        } = *self;

        if n_head > 0 {
            let (next, remaining) = slice.split_at(head_len);
            self.slice = remaining;
            self.n_head -= 1;
            Some(next)
        } else if n_tail > 0 {
            let (next, remaining) = slice.split_at(tail_len);
            self.slice = remaining;
            self.n_tail -= 1;
            Some(next)
        } else {
            None
        }
    }
}
