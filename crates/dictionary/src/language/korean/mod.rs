use crate::{Dictionary, Entry, EntryId, Etymologies, IndexedDictionary};
mod char_tree;
use char_tree::CharTree;
pub mod position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KoreanDictionary {
    pub entries: Vec<KoreanEntry>,
}

impl Dictionary for KoreanDictionary {
    fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    fn len(&self) -> usize {
        self.entries.len()
    }

    fn get_entry(&self, index: u32) -> Option<&dyn Entry> {
        let entry = self.entries.get(index as usize)?;
        Some(entry)
    }
}

impl KoreanDictionary {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_korean_entry(&mut self, entry: KoreanEntry) {
        self.entries.push(entry);
    }

    pub fn get_korean_entry(&self, index: EntryId) -> Option<&KoreanEntry> {
        let entry = self.entries.get(index as usize)?;
        Some(entry)
    }

    pub fn get_korean_entry_mut(&mut self, index: EntryId) -> Option<&mut KoreanEntry> {
        let entry = self.entries.get_mut(index as usize)?;
        Some(entry)
    }

    pub fn index(self) -> KoreanIndexedDictionary {
        KoreanIndexedDictionary {
            indexer: CharTree::new_by_korean_dictionary(&self),
            dictionary: self,
        }
    }
}

#[derive(Debug)]
pub struct KoreanIndexedDictionary {
    dictionary: KoreanDictionary,
    indexer: CharTree,
}

impl IndexedDictionary for KoreanIndexedDictionary {
    fn dictionary(&self) -> &dyn Dictionary {
        &self.dictionary
    }

    fn indexer(&self) -> &dyn crate::Indexer {
        &self.indexer
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KoreanEntry {
    pub word: String,
    pub variants: Vec<String>,
    pub associate_ids: Vec<EntryId>,
    pub definition: String,
    pub code: u32,
    pub etymologies: Etymologies,
    pub id: EntryId,
}

impl Entry for KoreanEntry {
    fn word(&self) -> &str {
        &self.word
    }

    fn id(&self) -> EntryId {
        self.id
    }

    fn string_description(&self) -> String {
        format!(
            "{}\n{}\n\n{}\n",
            self.word, self.etymologies, self.definition
        )
    }

    fn definition(&self) -> Option<&str> {
        Some(&self.definition)
    }

    fn etymologies(&self) -> Option<&Etymologies> {
        Some(&self.etymologies)
    }
}
