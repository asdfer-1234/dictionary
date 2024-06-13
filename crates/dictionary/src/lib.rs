#![allow(uncommon_codepoints)]
#![warn(missing_debug_implementations)]

pub mod etymology;
pub use etymology::*;
pub mod language;
pub use language::DictionaryDestination;

pub type EntryId = u32;

pub trait Dictionary
where
    Self: std::fmt::Debug,
{
    fn to_bytes(&self) -> Vec<u8>;
    fn len(&self) -> usize;
    fn get_entry(&self, index: EntryId) -> Option<&dyn Entry>;
}

pub trait IndexedDictionary
where
    Self: std::fmt::Debug,
{
    fn dictionary(&self) -> &dyn Dictionary;
    fn indexer(&self) -> &dyn Indexer;

    fn search(&self, query: &str) -> Vec<&dyn Entry> {
        self.indexer()
            .search(query)
            .iter()
            .map(|x| self.dictionary().get_entry(*x).unwrap())
            .collect()
    }
}

pub trait Indexer
where
    Self: std::fmt::Debug,
{
    fn search(&self, query: &str) -> Vec<EntryId>;
}

pub trait Entry: EntryClone
where
    Self: std::fmt::Debug,
{
    fn word(&self) -> &str;
    fn string_description(&self) -> String;
    fn definition(&self) -> Option<&str>;
    fn id(&self) -> EntryId;
    fn etymologies(&self) -> Option<&Etymologies>;
}

pub trait EntryClone {
    fn clone_box(&self) -> Box<dyn Entry>;
}

impl<T> EntryClone for T
where
    T: 'static + Entry + Clone,
{
    fn clone_box(&self) -> Box<dyn Entry> {
        Box::new(self.clone())
    }
}
/*
 * Rust is confusing sometimes isn't it,
 * with so many implicit things going on the background
 * it makes way too much misconceptions to newcomers like me
 */

impl Clone for Box<dyn Entry> {
    fn clone(&self) -> Box<dyn Entry> {
        self.clone_box()
    }
}
