use std::{cmp, iter::Chain, slice::Chunks};

super::ext!();

pub struct Parts<'a, T> {
    head: Chunks<'a, T>,
    tail: Chunks<'a, T>,
}

impl<'a, T> Parts<'a, T> {
    pub fn new(slice: &'a [T], n_parts: usize) -> Self {
        let len = slice.len();

        let tail_chunk_len = len / n_parts;
        let head_chunk_len = tail_chunk_len + 1;
        let n_head = len - (tail_chunk_len * n_parts);

        let midpoint = head_chunk_len * n_head;

        let (head, tail) = slice.split_at(midpoint);

        let head = head.chunks(cmp::max(head_chunk_len, 1));
        let tail = tail.chunks(cmp::max(tail_chunk_len, 1));

        Self { head, tail }
    }

    #[inline]
    // Storing a `Chain` directly led to decreased performance. In
    // some cases, the time needed to create and fully iterate
    // this type doubled.
    fn chained(&mut self) -> Chain<&mut Chunks<'a, T>, &mut Chunks<'a, T>> {
        let Self { head, tail } = self;
        head.chain(tail)
    }
}

impl<'a, T> Iterator for Parts<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        self.chained().next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let Self { head, tail } = self;

        // Copy-pasted from `Chain::size_hint` -- is there a
        // stdlib thing we can steal?

        let (head_lower, head_upper) = head.size_hint();
        let (tail_lower, tail_upper) = tail.size_hint();

        let lower = head_lower.saturating_add(tail_lower);

        let upper = match (head_upper, tail_upper) {
            (Some(x), Some(y)) => x.checked_add(y),
            _ => None,
        };

        (lower, upper)
    }

    // TODO: Delegate more methods?
}

// impl<'a, T> DoubleEndedIterator for Parts<'a, T> {
//     fn next_back(&mut self) -> Option<Self::Item> {
//         self.chained().next_back()
//     }

//     // TODO: Delegate more methods?
// }

// impl<'a, T> ExactSizeIterator for Parts<'a, T> {}

// impl<'a, T> FusedIterator for Parts<'a, T> {}
