#[derive(Debug)]
struct MonoStack<T>
where
    T: Ord,
{
    ord: std::cmp::Ordering,
    buf: Vec<T>,
}

impl<T: Ord> MonoStack<T> {
    fn push(&mut self, item: T) -> impl Iterator<Item = T> + '_ {
        let mut ptr = self.buf.len();
        if self.buf.len() > 0 {
            ptr -= 1;
            while ptr > 0 && self.buf[ptr].cmp(&item) == self.ord {
                ptr -= 1;
            }
        }
        self.buf.splice(ptr.., [item])
    }
}
