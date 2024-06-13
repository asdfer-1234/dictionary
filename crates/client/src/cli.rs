use dictionary::IndexedDictionary;
use std::io;
use std::io::Write;

use crate::{file::read_indexed_dictionaries, Config, MainError};

fn search(dicts: &Vec<Box<dyn IndexedDictionary>>, query: &str) {
    for dict in dicts {
        let result = dict.search(query);
        for e in result {
            println!("{}", e.string_description());
        }
    }
}

pub fn run(config: &Config) -> Result<(), MainError> {
    let dicts = read_indexed_dictionaries(&config.dictionary_directory);
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        let mut buffer = String::new();
        print!("> ");
        stdout.flush().unwrap();
        stdin.read_line(&mut buffer).unwrap();

        let command = buffer.trim();

        if command == "quit" {
            break;
        } else {
            search(&dicts, command);
        }

        println!("{}", buffer);
    }
    Ok(())
}
