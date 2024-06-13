use crate::EntryId;
use enum_map::{enum_map, EnumMap};

use hangeul::{Choseong, Jamo, Jongseong, Jungseong, Syllable};
use serde::{Deserialize, Serialize};

pub trait CharNode {
    type Indexer: Copy + Jamo;
    type Next: CharNode;

    fn new() -> Self;

    fn get(&self, index: Self::Indexer) -> Option<&Self::Next>;
    fn get_mut(&mut self, index: Self::Indexer) -> Option<&mut Self::Next>;
    fn set(&mut self, index: Self::Indexer, next: Self::Next);

    fn insert_index_if_none(&mut self, index: Self::Indexer) {
        if let None = self.get(index) {
            self.set(index, Self::Next::new());
        }
    }

    fn get_or_insert(&mut self, index: Self::Indexer) -> &mut Self::Next {
        self.insert_index_if_none(index);
        self.get_mut(index).unwrap()
    }
}

macro_rules! char_node_get_set {
    () => {
        fn get(&self, index: Self::Indexer) -> Option<&Self::Next> {
            self.next[index].as_deref()
        }

        fn get_mut(&mut self, index: Self::Indexer) -> Option<&mut Self::Next> {
            use std::borrow::BorrowMut;

            match self.next[index].as_mut() {
                Some(b) => Some(b.borrow_mut()),
                _ => None,
            }
        }

        fn set(&mut self, index: Self::Indexer, next: Self::Next) {
            self.next[index] = Some(Box::new(next));
        }
    };
}

// Honestly I have no idea what I did here.
// Iterators are confusing,
// especially when I'm just referencing code from enum-map without proper insight.
// but it works, it works!
macro_rules! enum_map_into_iterator {
    ($node: ty, $key: ty, $value: ty) => {
        impl IntoIterator for $node {
            type Item = ($key, $value);
            type IntoIter = enum_map::IntoIter<$key, $value>;

            fn into_iter(self) -> Self::IntoIter {
                self.next.into_iter()
            }
        }

        impl<'a> IntoIterator for &'a $node {
            type Item = ($key, &'a $value);
            type IntoIter = enum_map::Iter<'a, $key, $value>;

            fn into_iter(self) -> Self::IntoIter {
                self.next.iter()
            }
        }

        impl<'a> IntoIterator for &'a mut $node {
            type Item = ($key, &'a mut $value);
            type IntoIter = enum_map::IterMut<'a, $key, $value>;

            fn into_iter(self) -> Self::IntoIter {
                self.next.iter_mut()
            }
        }
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChoseongNode {
    pub entry_ids: Vec<EntryId>,
    pub next: EnumMap<Choseong, Option<Box<JungseongNode>>>,
}

impl CharNode for ChoseongNode {
    type Indexer = Choseong;
    type Next = JungseongNode;

    fn new() -> Self {
        Self {
            next: enum_map! {_ => None},
            entry_ids: Vec::new(),
        }
    }

    char_node_get_set! {}
}

impl ChoseongNode {
    pub fn get_by_syllable(&self, syllable: Syllable) -> Option<&ChoseongNode> {
        self.get(syllable.choseong)?
            .get(syllable.jungseong)?
            .get(syllable.jongseong)
    }

    pub fn get_by_syllable_mut(&mut self, syllable: Syllable) -> Option<&mut ChoseongNode> {
        self.get_mut(syllable.choseong)?
            .get_mut(syllable.jungseong)?
            .get_mut(syllable.jongseong)
    }

    pub fn get_or_insert_by_syllable(&mut self, syllable: Syllable) -> &mut ChoseongNode {
        self.get_or_insert(syllable.choseong)
            .get_or_insert(syllable.jungseong)
            .get_or_insert(syllable.jongseong)
    }

    pub fn add_entry_id(&mut self, entry_id: EntryId) {
        self.entry_ids.push(entry_id);
    }

    pub fn iter_syllables(&self) -> ChoseongNodeSyllableIterator {
        ChoseongNodeSyllableIterator {
            syllable: Some(Syllable::first()),
            node: self,
        }
    }
}

enum_map_into_iterator! {ChoseongNode, Choseong, Option<Box<JungseongNode>>}

#[derive(Debug)]
pub struct ChoseongNodeSyllableIterator<'a> {
    syllable: Option<Syllable>,
    node: &'a ChoseongNode,
}

impl<'a> ChoseongNodeSyllableIterator<'a> {
    fn pointing_valid_syllable(&self) -> bool {
        if let Some(x) = self.syllable {
            matches!(self.node.get_by_syllable(x), Some(_))
        } else {
            false
        }
    }

    fn to_next(&mut self) {
        self.syllable = match self.syllable {
            Some(i) => i.next_matching(
                |x| matches!(self.node.get(x), Some(_)),
                |x, y| matches!(self.node.get(x).unwrap().get(y), Some(_)),
                |x| matches!(self.node.get_by_syllable(x), Some(_)),
            ),
            None => None,
        };
    }
}

impl<'a> Iterator for ChoseongNodeSyllableIterator<'a> {
    type Item = &'a ChoseongNode;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.pointing_valid_syllable() {
            self.to_next();
            println!("wfuytnwfuytn");
        }
        let ret = match self.syllable {
            Some(x) => self.node.get_by_syllable(x),
            None => None,
        };
        self.syllable = self.syllable?.next();
        println!("{}", self.syllable?);
        self.to_next();
        ret
    }
}

struct ChoseongNodeEntryIterator<'a> {
    path: Vec<ChoseongNodeSyllableIterator<'a>>,
    node: &'a ChoseongNode,
}

impl<'a> Iterator for ChoseongNodeEntryIterator<'a> {
    type Item = ChoseongNode;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JungseongNode {
    pub next: EnumMap<Jungseong, Option<Box<JongseongNode>>>,
}

impl CharNode for JungseongNode {
    type Indexer = Jungseong;
    type Next = JongseongNode;

    fn new() -> Self {
        Self {
            next: enum_map! {_ => None},
        }
    }

    char_node_get_set! {}
}

enum_map_into_iterator! {JungseongNode, Jungseong, Option<Box<JongseongNode>>}

#[derive(Debug, Serialize, Deserialize)]
pub struct JongseongNode {
    pub next: EnumMap<Jongseong, Option<Box<ChoseongNode>>>,
}

impl CharNode for JongseongNode {
    type Indexer = Jongseong;
    type Next = ChoseongNode;

    fn new() -> Self {
        Self {
            next: enum_map! {_ => None},
        }
    }

    char_node_get_set! {}
}

enum_map_into_iterator! {JongseongNode, Jongseong, Option<Box<ChoseongNode>>}
