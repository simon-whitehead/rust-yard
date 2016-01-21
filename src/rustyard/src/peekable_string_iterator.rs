use std::iter;
use std::str;

pub struct PeekableStringIterator<'a> {
    raw_input: String,
    iter: iter::Peekable<str::Chars<'a>>
}

impl<'a> PeekableStringIterator<'a> {
    fn new(raw_input: &str) -> PeekableStringIterator {
        PeekableStringIterator {
            raw_input: raw_input.to_string(),
            iter: raw_input.chars().peekable()
        }
    }

    pub fn advance(&mut self) {
        self.iter.next();
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }
}
