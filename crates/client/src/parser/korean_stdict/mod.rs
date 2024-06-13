use dictionary::language::korean;
use dictionary::Dictionary;
use dictionary::EntryId;
use korean::{KoreanDictionary, KoreanEntry};
mod json;
mod original_language;
use self::original_language::json_to_etymologies;
use super::ParseError;

type TargetCode = u32;

#[derive(Debug)]
pub enum IdConnection {
    Subscribing(Vec<EntryId>),
    Providing(EntryId),
}

#[derive(Debug)]
pub struct IdConnector {
    connections: Vec<IdConnection>,
    buffer: Vec<(EntryId, EntryId)>,
}

impl IdConnector {
    fn new() -> Self {
        Self {
            connections: vec![],
            buffer: vec![],
        }
    }

    fn extend(&mut self, index: TargetCode) {
        if self.connections.len() as u32 <= index {
            self.connections
                .resize_with((index + 1) as usize, || IdConnection::Subscribing(vec![]));
        }
    }

    pub fn subscribe(&mut self, index: TargetCode, id: EntryId) {
        self.extend(index);
        match &mut self.connections[index as usize] {
            IdConnection::Subscribing(vec) => vec.push(id),
            IdConnection::Providing(p) => self.buffer.push((*p, id)),
        }
    }

    pub fn provide(&mut self, index: TargetCode, id: EntryId) {
        self.extend(index);
        match &mut self.connections[index as usize] {
            IdConnection::Providing(_) => {}
            IdConnection::Subscribing(vec) => {
                for i in vec {
                    self.buffer.push((id, *i));
                }

                self.connections[index as usize] = IdConnection::Providing(id);
            }
        }
    }

    pub fn flush(&mut self) -> Vec<(EntryId, EntryId)> {
        let mut ret = vec![];
        take_mut::take(&mut self.buffer, |mut x| {
            ret = x;
            x = vec![];
            x
        });
        ret
    }
}

fn parse_full_json(json: json::Json) -> KoreanDictionary {
    let mut dict = KoreanDictionary::new();
    let mut id_connector = IdConnector::new();
    for item in &json.channel.item {
        for pos_info in &item.word_info.pos_info {
            for comm_pattern_info in &pos_info.comm_pattern_info {
                for sense_info in &comm_pattern_info.sense_info {
                    {
                        let id = dict.len() as EntryId;
                        let code = item.target_code;
                        let word_info = &item.word_info;
                        let temporary_replacement_vector = vec![];
                        let relation_info = match &word_info.relation_info {
                            Some(x) => x,
                            None => &temporary_replacement_vector,
                        };

                        for i in relation_info {
                            let code: u32 = i.link_target_code.parse().unwrap();
                            id_connector.subscribe(code, id);
                        }
                        id_connector.provide(code, id);

                        dict.add_korean_entry(KoreanEntry {
                            word: word_info.word.clone(),
                            variants: vec![],
                            associate_ids: vec![],
                            definition: sense_info.definition.clone(),
                            code: item.target_code,
                            etymologies: json_to_etymologies(&word_info.original_language_info),
                            id: dict.len() as EntryId,
                        });

                        for (from_id, to_id) in id_connector.flush() {
                            dict.get_korean_entry_mut(from_id)
                                .unwrap()
                                .associate_ids
                                .push(to_id);
                            dict.get_korean_entry_mut(to_id)
                                .unwrap()
                                .associate_ids
                                .push(from_id);
                        }
                    };
                }
            }
        }
    }
    dict
}

pub fn parse(args: &[String]) -> Result<KoreanDictionary, ParseError> {
    let mut full_json = json::empty();
    for arg in args {
        println!("Parsing {}", arg);
        let json_string = std::fs::read_to_string(arg)?;
        let json: json::Json = serde_json::from_str(&json_string)?;
        full_json.append(json);
    }
    let dict = parse_full_json(full_json);
    Ok(dict)
}
