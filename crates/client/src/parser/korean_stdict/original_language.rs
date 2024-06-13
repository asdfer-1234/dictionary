use super::json;
use dictionary::DictionaryDestination;
use dictionary::{Etymologies, Etymology, EtymologySegment};

#[derive(Debug)]
enum Symbol {
    ParallelSeparator,
    Segment(EtymologySegment),
}

impl Symbol {
    fn dictionary_destination(
        dictionary_destination: DictionaryDestination,
        string: String,
    ) -> Self {
        Self::Segment(EtymologySegment::DictionaryDestination(
            dictionary_destination,
            string,
        ))
    }

    fn from_json(json: &json::OriginalLanguageInfo) -> Option<Symbol> {
        use DictionaryDestination::*;

        let o = json.original_language.clone();

        Some(match json.language_type.as_str() {
            "/(병기)" => Symbol::ParallelSeparator,
            "한자" => Self::dictionary_destination(Han, o?),
            "고유어" => Self::dictionary_destination(Korean, o?),
            "영어" => Self::dictionary_destination(English, o?),
            "일본어" => Self::dictionary_destination(Japanese, o?),
            "중국어" => Self::dictionary_destination(Chinese, o?),
            "프랑스어" => Self::dictionary_destination(French, o?),
            "독일어" => Self::dictionary_destination(German, o?),
            "이탈리아어" => Self::dictionary_destination(Italian, o?),
            "그리스어" => Self::dictionary_destination(Greek, o?),
            "라틴어" => Self::dictionary_destination(Latin, o?),
            "러시아어" => Self::dictionary_destination(Russian, o?),
            "말레이어" => Self::dictionary_destination(Malay, o?),
            "아랍어" => Self::dictionary_destination(Arab, o?),
            "체코어" => Self::dictionary_destination(Czech, o?),
            "히브리어" => Self::dictionary_destination(Hebrew, o?),
            "몽골어" => Self::dictionary_destination(Mongolian, o?),
            "산스크리트어" => Self::dictionary_destination(Sanskrit, o?),
            "에스파냐어" => Self::dictionary_destination(Spanish, o?),
            "헝가리어" => Self::dictionary_destination(Hungarian, o?),
            "루마니아어" => Self::dictionary_destination(Romanian, o?),
            "네덜란드어" => Self::dictionary_destination(Dutch, o?),
            "불가리아어" => Self::dictionary_destination(Bulgarian, o?),
            "페르시아어" => Self::dictionary_destination(Persian, o?),
            "포르투갈어" => Self::dictionary_destination(Portuguese, o?),
            "힌디어" => Self::dictionary_destination(Hindi, o?),
            "핀란드어" => Self::dictionary_destination(Finnish, o?),
            "베트남어" => Self::dictionary_destination(Vietnamese, o?),
            "스웨덴어" => Self::dictionary_destination(Swedish, o?),
            "타이어" => Self::dictionary_destination(Thai, o?),
            "인도네시아어" => Self::dictionary_destination(Indonesian, o?),
            "터키어" => Self::dictionary_destination(Turkish, o?),
            "세르보·크로아트어" => Self::dictionary_destination(Serbian, o?),
            "노르웨이어" => Self::dictionary_destination(Norwegian, o?),
            "안 밝힘" => Symbol::Segment(EtymologySegment::Unknown(o?)),
            "기타어" => Symbol::Segment(EtymologySegment::Unknown(o?)),
            _ => return None,
        })
    }
}

pub fn json_to_etymologies(json: &Option<Vec<json::OriginalLanguageInfo>>) -> Etymologies {
    match json {
        Some(vec) => original_language_infos_to_etymologies(vec),
        None => Etymologies::new(),
    }
}

fn original_language_infos_to_etymologies(infos: &[json::OriginalLanguageInfo]) -> Etymologies {
    let mut etymologies = Etymologies::new();
    let mut etymology = Etymology::new();

    for json_etymology in infos {
        let symbol = match Symbol::from_json(json_etymology) {
            Some(s) => s,
            None => return Etymologies::new(),
        };

        match symbol {
            Symbol::ParallelSeparator => {
                etymologies.push(etymology);
                etymology = Etymology::new();
            }
            Symbol::Segment(s) => {
                etymology.push(s);
            }
        }
    }
    etymologies.push(etymology);
    etymologies
}
