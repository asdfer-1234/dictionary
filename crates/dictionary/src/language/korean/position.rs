#[derive(Debug)]
pub enum Position {
    Noun,
    Pronoun,
    Numeral,
    Postposition,
    Verb,
    Adjective,
    Determinative,
    Adverb,
    Interjection,
    Conjunction,
    DependentNoun,
    SecondaryVerb,
    SecondaryAdjective,
    Stem,
    None,
    Undefined(String),
}

impl Position {
    pub fn from_str(str: &str) -> Self {
        match str {
            "명사" => Self::Noun,
            "대명사" => Self::Pronoun,
            "수사" => Self::Numeral,
            "조사" => Self::Postposition,
            "동사" => Self::Verb,
            "형용사" => Self::Adjective,
            "관형사" => Self::Determinative,
            "부사" => Self::Adverb,
            "감탄사" => Self::Interjection,
            "접사" => Self::Conjunction,
            "의존 명사" => Self::DependentNoun,
            "보조 동사" => Self::SecondaryVerb,
            "보조 형용사" => Self::SecondaryAdjective,
            "어미" => Self::Stem,
            "품사 없음" => Self::None,
            _ => Self::Undefined(str.to_string()),
        }
    }
    fn to_string(&self) -> String {
        match self {
            Self::Noun => "명사",
            Self::Pronoun => "대명사",
            Self::Numeral => "수사",
            Self::Postposition => "조사",
            Self::Verb => "동사",
            Self::Adjective => "형용사",
            Self::Determinative => "관형사",
            Self::Adverb => "부사",
            Self::Interjection => "감탄사",
            Self::Conjunction => "접사",
            Self::DependentNoun => "의존 명사",
            Self::SecondaryVerb => "보조 동사",
            Self::SecondaryAdjective => "보저 형용사",
            Self::Stem => "어미",
            Self::None => "품사 없음",
            Self::Undefined(s) => &s,
        }
        .to_owned()
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
