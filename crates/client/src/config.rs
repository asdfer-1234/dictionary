use std::env;
use std::path::PathBuf;

const DEFAULT_CONFIG_DIRECTORY_NAME: &str = "dictionary";
const DEFAULT_DICTIONARIES_DIRECTORY_NAME: &str = "dictionaries";

#[derive(Debug, Clone)]
pub struct Config {
    pub dictionary_directory: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dictionary_directory: default_dictionary_directory(),
        }
    }
}

fn home_directory() -> PathBuf {
    PathBuf::from(env::var("HOME").unwrap())
}

fn home_config_directory() -> PathBuf {
    let xdg_config_home = env::var("XDG_CONFIG_HOME");
    if let Ok(i) = xdg_config_home {
        return PathBuf::from(i);
    }

    let home = home_directory();
    PathBuf::from_iter([home, ".config".into()])
}

fn default_config_directory() -> PathBuf {
    PathBuf::from_iter([
        home_config_directory(),
        DEFAULT_CONFIG_DIRECTORY_NAME.into(),
    ])
}

fn default_dictionary_directory() -> PathBuf {
    PathBuf::from_iter([
        default_config_directory(),
        DEFAULT_DICTIONARIES_DIRECTORY_NAME.into(),
    ])
}
