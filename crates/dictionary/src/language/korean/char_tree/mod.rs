use super::{Entry, KoreanDictionary};
use crate::{EntryId, Indexer};
use hangeul::Syllable;
use serde::{Deserialize, Serialize};
use std::iter;

mod char_node;
use char_node::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct CharTree {
    root: ChoseongNode,
}

impl CharTree {
    pub fn new() -> Self {
        Self {
            root: ChoseongNode::new(),
        }
    }

    pub fn index_id(&mut self, query: &Vec<Syllable>, id: EntryId) {
        let node = self.get_or_insert_syllables(query);
        node.add_entry_id(id);
    }

    pub fn index_korean_dictionary(&mut self, dictionary: &KoreanDictionary) {
        for entry in &dictionary.entries {
            let id = entry.id();

            let main_iterator = iter::once(Syllable::from_str(entry.word()));
            let variant_iterator = entry.variants.iter().map(|x| Syllable::from_str(x));

            let all_iterator = main_iterator.chain(variant_iterator);

            for syllables in all_iterator {
                self.index_id(&syllables, id);
            }
        }
    }

    pub fn new_by_korean_dictionary(dictionary: &KoreanDictionary) -> CharTree {
        let mut char_tree = Self::new();
        char_tree.index_korean_dictionary(dictionary);
        char_tree
    }

    pub fn get(&self, syllables: &[Syllable]) -> &[EntryId] {
        if let Some(node) = self.query_choseong_node(syllables) {
            &node.entry_ids
        } else {
            &[] // Empty set. Nice.
        }
    }

    fn query_choseong_node(&self, syllables: &[Syllable]) -> Option<&ChoseongNode> {
        let mut node = &self.root;
        for s in syllables {
            node = node.get_by_syllable(*s)?;
        }
        Some(node)
    }

    fn get_or_insert_syllables(&mut self, syllables: &[Syllable]) -> &mut ChoseongNode {
        let mut node = &mut self.root;
        for s in syllables {
            node = node.get_or_insert_by_syllable(*s);
        }
        node
    }
}

impl Indexer for CharTree {
    fn search(&self, query: &str) -> Vec<EntryId> {
        let syllables = hangeul::Syllable::from_str(query);
        self.get(&syllables).to_vec()
    }
}
