use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OriginalLanguageInfo {
    pub original_language: Option<String>,
    pub language_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct PronunciationInfo {
    pub pronunciation: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ConjugationInfo {
    pub conjugation: String,
    pub pronunciation_info: Option<Vec<PronunciationInfo>>,
}

#[derive(Serialize, Deserialize)]
pub struct AbbreviationInfo {
    pub abbreviation: String,
    pub pronunciation_info: Option<Vec<PronunciationInfo>>,
}

#[derive(Serialize, Deserialize)]
pub struct ConjuInfo {
    pub conjugation_info: ConjugationInfo,
    pub abbreviation_info: Option<AbbreviationInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct RelationInfo {
    pub word: String,
    pub r#type: String,
    pub link_target_code: String,
    pub link: String,
}

#[derive(Serialize, Deserialize)]
pub struct LexicalInfo {
    pub word: Option<String>,
    pub unit: String,
    pub r#type: String,
    pub link_target_code: Option<String>,
    pub link: String,
}

#[derive(Serialize, Deserialize)]
pub struct PatternInfo {
    pub pattern: String,
}

#[derive(Serialize, Deserialize)]
pub struct SensePatternInfo {
    pub pattern: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SenseGrammarInfo {
    pub grammar: String,
}

#[derive(Serialize, Deserialize)]
pub struct CatInfo {
    pub cat: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExampleInfo {
    pub example: Option<String>,
    pub source: Option<String>,
    pub origin: Option<String>,
    pub translation: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TranslationInfo {
    pub translation: String,
    pub language_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct MultimediaInfo {
    pub label: Option<String>,
    pub r#type: String,
    pub link: String,
}

#[derive(Serialize, Deserialize)]
pub struct SenseInfo {
    pub sense_code: u32,
    pub r#type: String,
    pub definition: String,
    pub definition_original: String,
    pub scientific_name: Option<String>,
    pub sense_pattern_info: Option<Vec<SensePatternInfo>>,
    pub sense_grammar_info: Option<Vec<SenseGrammarInfo>>,
    pub cat_info: Option<Vec<CatInfo>>,
    pub example_info: Option<Vec<ExampleInfo>>,
    pub translation_info: Option<TranslationInfo>,
    pub multimedia_info: Option<Vec<MultimediaInfo>>,
    pub lexical_info: Option<Vec<LexicalInfo>>,
}

#[derive(Serialize, Deserialize)]
pub struct CommPatternInfo {
    pub comm_pattern_code: String,
    pub pattern_info: Option<PatternInfo>,
    pub lexical_info: Option<Vec<LexicalInfo>>,
    pub sense_info: Vec<SenseInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct PosInfo {
    pub pos_code: String,
    pub pos: String,
    pub lexical_info: Option<Vec<LexicalInfo>>,
    pub comm_pattern_info: Vec<CommPatternInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct WordInfo {
    pub word: String,
    pub word_unit: String,
    pub word_type: String,
    pub original_language_info: Option<Vec<OriginalLanguageInfo>>,
    pub pronunciation_info: Option<Vec<PronunciationInfo>>,
    pub conju_info: Option<Vec<ConjuInfo>>,
    pub relation_info: Option<Vec<RelationInfo>>,
    pub origin: Option<String>,
    pub allomorph: Option<String>,
    pub lexical_info: Option<Vec<LexicalInfo>>,
    pub pos_info: Vec<PosInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub target_code: u32,
    pub word_info: WordInfo,
}

#[derive(Serialize, Deserialize)]
pub struct Channel {
    pub total: u32,
    pub item: Vec<Item>,
}

#[derive(Serialize, Deserialize)]
pub struct Json {
    pub channel: Channel,
}

impl Json {
    pub fn new() -> Self {
        Self {
            channel: Channel {
                total: 0,
                item: vec![],
            },
        }
    }

    pub fn append(&mut self, mut json: Json) {
        self.channel.item.append(&mut json.channel.item);
    }
}

pub fn empty() -> Json {
    Json::new()
}
