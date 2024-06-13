use serde::{Deserialize, Serialize};

pub mod korean;
//pub mod english;
//pub mod german;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum DictionaryDestination {
    Latin,
    English,
    German,
    Dutch,
    Italian,
    French,
    Spanish,
    Portuguese,
    Czech,
    Romanian,
    Danish,
    Swedish,
    Norwegian,
    Finnish,
    Icelandic,
    Slavic,
    Malay,
    Hungarian,
    Turkish,
    Indonesian,

    Russian,
    Mongolian,
    Serbian,
    Bulgarian,

    Greek,

    Han,
    Chinese,
    Japanese,
    Korean,
    Vietnamese,

    Thai,

    Arab,
    Hebrew,
    Sanskrit,
    Persian,
    Hindi,
}
// I'm no language origin expert
