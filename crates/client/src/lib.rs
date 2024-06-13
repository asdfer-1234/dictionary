#![warn(missing_debug_implementations)]
pub mod config;
pub use config::Config;
pub mod parser;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
pub mod main_error;
pub use main_error::MainError;
//pub mod gui;
pub mod cli;
pub mod file;
pub mod gui;

#[derive(Debug)]
pub struct Client {
    config: Config,
}

impl Client {
    pub fn new_and_run(args: &[String]) -> Result<(), main_error::MainError> {
        let client = Client {
            config: Config::default(),
        };
        client.run(args)
    }

    fn run(self, args: &[String]) -> Result<(), main_error::MainError> {
        if args.len() == 0 {
            self.gui(args)?;
            return Ok(());
        }
        match args[0].as_str() {
            "gui" => self.gui(&args[1..]),
            "parse" => self.parse(&args[1..]),
            _ => {
                println!("\"{}\" is not a valid subcommand.", args[0]);
                Ok(())
            }
        }
    }

    fn gui(&self, args: &[String]) -> Result<(), main_error::MainError> {
        if args.len() != 0 {
            return Err(main_error::MainError::Command);
        }
        gui::run(&self.config);
        Ok(())
    }

    fn parse(&self, args: &[String]) -> Result<(), main_error::MainError> {
        use bincode::serialize;

        let parser_name = &args[0];
        match parser_name.as_str() {
            "korean_stdict" => self.save_file_as_dictionary(
                &serialize(&parser::korean_stdict::parse(&args[1..])?)?,
                "korean-stdict.ko-dict",
            ),
            _ => {
                println!("\"{}\" is not a valid parser.", args[0]);
                return Err(main_error::MainError::Command);
            }
        };
        Ok(())
    }

    fn save_file_as_dictionary(&self, bytes: &[u8], name: &str) {
        let file_name = name.to_string();
        fs::create_dir_all(&self.config.dictionary_directory).unwrap();
        let file_path = self.config.dictionary_directory.join(&file_name);
        let mut file = File::create(file_path).unwrap();
        file.write_all(bytes).unwrap();
    }
}
