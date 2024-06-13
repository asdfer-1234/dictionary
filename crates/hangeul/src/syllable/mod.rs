use crate::jamo::{Choseong, Jamo, JamoIndex, Jongseong, Jungseong};
use serde::{Deserialize, Serialize};

pub mod iter;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Syllable {
    pub choseong: Choseong,
    pub jungseong: Jungseong,
    pub jongseong: Jongseong,
}

impl Syllable {
    pub fn new(choseong: Choseong, jungseong: Jungseong, jongseong: Jongseong) -> Self {
        Self {
            choseong,
            jungseong,
            jongseong,
        }
    }

    pub fn first() -> Self {
        Self::new(Choseong::ㄱ, Jungseong::ㅏ, Jongseong::None)
    }

    pub fn from_char(s: char) -> Option<Self> {
        if !('가'..='힣').contains(&s) {
            return None;
        }
        let i = (s as JamoIndex) - ('가' as JamoIndex);

        let l1 = Jongseong::length() as JamoIndex;
        let l2 = Jungseong::length() as JamoIndex;
        let l3 = Choseong::length() as JamoIndex;

        let jongseong_index = i % l1;
        let jungseong_index = i / l1 % l2;
        let choseong_index = i / l1 / l2 % l3;

        Some(Self {
            choseong: Choseong::from_index(choseong_index)?,
            jungseong: Jungseong::from_index(jungseong_index)?,
            jongseong: Jongseong::from_index(jongseong_index)?,
        })
    }

    pub fn from_str(s: &str) -> Vec<Syllable> {
        let mut construction = Vec::new();
        for c in s.chars() {
            if let Some(s) = Self::from_char(c) {
                construction.push(s);
            }
        }
        construction
    }

    pub fn char(self) -> char {
        char::from_u32(
            '가' as u32
                + counting_system_construction(vec![
                    (
                        self.jongseong.index() as u32,
                        Jongseong::length().try_into().unwrap(),
                    ),
                    (
                        self.jungseong.index() as u32,
                        Jungseong::length().try_into().unwrap(),
                    ),
                    (
                        self.choseong.index() as u32,
                        Choseong::length().try_into().unwrap(),
                    ),
                ]),
        )
        .unwrap()
    }

    pub fn next(self) -> Option<Syllable> {
        self.next_jongseong()
    }

    pub fn next_jongseong(self) -> Option<Syllable> {
        match self.jongseong.next() {
            Some(x) => Some(Syllable::new(self.choseong, self.jungseong, x)),
            None => self.next_jungseong(),
        }
    }

    pub fn next_jungseong(self) -> Option<Syllable> {
        match self.jungseong.next() {
            Some(x) => Some(Syllable::new(self.choseong, x, Jongseong::None)),
            None => self.next_choseong(),
        }
    }

    pub fn next_choseong(self) -> Option<Syllable> {
        match self.choseong.next() {
            Some(x) => Some(Syllable::new(x, Jungseong::ㅏ, Jongseong::None)),
            None => None,
        }
    }

    pub fn next_matching<
        ChoseongCondition: FnMut(Choseong) -> bool,
        JungseongCondition: FnMut(Choseong, Jungseong) -> bool,
        SyllableCondition: FnMut(Syllable) -> bool,
    >(
        self,
        mut choseong_condition: ChoseongCondition,
        mut jungseong_condition: JungseongCondition,
        mut syllable_condition: SyllableCondition,
    ) -> Option<Syllable> {
        let mut s = self.next()?;

        loop {
            if choseong_condition(self.choseong) {
                if jungseong_condition(self.choseong, self.jungseong) {
                    if syllable_condition(self) {
                        return Some(self);
                    } else {
                        s = s.next_jongseong()?;
                    }
                } else {
                    s = s.next_jungseong()?;
                }
            } else {
                s = s.next_choseong()?;
            }
        }
    }
}

impl std::fmt::Display for Syllable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.char())
    }
}

fn counting_system_construction(counts: Vec<(u32, u32)>) -> u32 {
    let mut current_base = 1u32;
    let mut total = 0u32;
    for (digit, base) in counts {
        total += current_base * digit;
        current_base *= base;
    }
    total
}
