pub struct StringChunksIter<'a> {
    s: &'a str,
    idx: usize,
    chunk_size: usize,
}

impl<'a> StringChunksIter<'a> {
    pub fn new(s: &'a str, chunk_size: usize) -> Self {
        Self {
            s,
            idx: 0,
            chunk_size,
        }
    }
}

impl<'a> Iterator for StringChunksIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = self
            .s
            .chars()
            .skip(self.idx)
            .take(self.chunk_size)
            .collect::<String>();

        if chunk.is_empty() {
            return None;
        }

        self.idx += self.chunk_size;

        Some(chunk)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::strings::StringChunksIter;

    #[test]
    fn chunk_1() {
        let s = "ABCDEFG";
        let mut iter = StringChunksIter::new(s, 2);

        let w = iter.next().unwrap();
        assert_eq!(w, "AB");
        let w = iter.next().unwrap();
        assert_eq!(w, "CD");
        let w = iter.next().unwrap();
        assert_eq!(w, "EF");
        let w = iter.next().unwrap();
        assert_eq!(w, "G");
    }

    #[test]
    fn chunk_2() {
        let s = "ABCDEFG";
        let mut iter = StringChunksIter::new(s, 3);

        let w = iter.next().unwrap();
        assert_eq!(w, "ABC");
        let w = iter.next().unwrap();
        assert_eq!(w, "DEF");
        let w = iter.next().unwrap();
        assert_eq!(w, "G");
    }
}
