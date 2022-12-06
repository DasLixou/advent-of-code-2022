use std::collections::VecDeque;

#[derive(Debug, Default, Clone)]
pub struct Stack {
    inner: VecDeque<char>,
}

impl Stack {
    #[inline(always)]
    pub fn put(&mut self, c: char) {
        self.inner.push_front(c);
    }

    #[inline(always)]
    pub fn take(&mut self) -> char {
        self.inner.pop_front().expect("Stack empty :c")
    }
}
