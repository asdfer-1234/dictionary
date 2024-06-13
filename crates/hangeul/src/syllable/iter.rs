use super::Syllable;

pub struct SyllableIterator {
    current: Option<Syllable>,
}

impl Iterator for SyllableIterator {
    type Item = Syllable;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.current;
        self.current = match self.current {
            Some(x) => x.next(),
            None => None,
        };
        ret
    }
}
