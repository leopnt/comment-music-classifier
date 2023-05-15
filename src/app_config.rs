use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    directories: Vec<Vec<String>>,
}

impl ::std::default::Default for AppConfig {
    fn default() -> Self {
        let default_directories: Vec<Vec<String>> = vec![
            vec!["ATTACK", "DECAY", "SUSTAIN", "RELEASE"],
            vec!["DARK", "NEUTRAL", "BRIGHT"],
            vec!["DISCO", "ELECTRO", "HOUSE", "ROCK", "TECHNO", "TRANCE"],
        ]
        .iter()
        .map(|row| row.iter().map(|&class| class.to_string()).collect())
        .collect();

        Self {
            directories: default_directories,
        }
    }
}
