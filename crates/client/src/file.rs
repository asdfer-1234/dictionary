use bincode::deserialize;
use dictionary::language::korean::*;
use dictionary::IndexedDictionary;
use std::fs;
use std::path::Path;

fn parse_korean_indexed_dictionary(bytes: &[u8]) -> Option<Box<dyn IndexedDictionary>> {
    let korean_dict: KoreanDictionary = deserialize(bytes).ok()?;
    Some(Box::new(korean_dict.index()))
}

pub fn read_indexed_dictionary(path: &Path) -> Option<Box<dyn IndexedDictionary>> {
    let parser = match path.extension()?.to_str()? {
        "ko-dict" => parse_korean_indexed_dictionary,
        _ => return None,
    };

    parser(&fs::read(&path).ok()?)
}

pub fn read_indexed_dictionaries(dictionaries_path: &Path) -> Vec<Box<dyn IndexedDictionary>> {
    let mut dicts = vec![];

    if let Err(_) = fs::create_dir_all(dictionaries_path) {
        println!("Failed to create path {} which is the path that this program stores its dictionary files.", dictionaries_path.to_string_lossy());
    }

    for dir_entry in dictionaries_path.read_dir().unwrap() {
        let path = dir_entry.unwrap().path();
        let dict_option = read_indexed_dictionary(&path);
        if let Some(dict) = dict_option {
            dicts.push(dict);
        }
    }
    dicts
}
