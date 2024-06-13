use super::DictionaryDestination;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EtymologySegment {
    DictionaryDestination(DictionaryDestination, String),
    Unknown(String),
}

impl Display for EtymologySegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EtymologySegment::DictionaryDestination(_, s) => s.fmt(f),
            EtymologySegment::Unknown(s) => s.fmt(f),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Etymology {
    segments: Vec<EtymologySegment>,
}

impl Etymology {
    pub fn new() -> Etymology {
        Self { segments: vec![] }
    }

    pub fn push(&mut self, item: EtymologySegment) {
        self.segments.push(item);
    }
}

impl Display for Etymology {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in &self.segments {
            write!(f, "{}", s)?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Etymologies {
    etymologies: Vec<Etymology>,
}

impl Etymologies {
    pub fn new() -> Self {
        Self {
            etymologies: vec![],
        }
    }

    pub fn push(&mut self, item: Etymology) {
        self.etymologies.push(item);
    }
}

impl Display for Etymologies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            &self.etymologies.iter().fold(String::new(), |init, x| {
                if init == String::new() {
                    init.to_string() + &x.to_string()
                } else {
                    String::from(" | ") + &init + &x.to_string()
                }
            })
        )?;
        Ok(())
    }
}
