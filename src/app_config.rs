use serde_derive::{Deserialize, Serialize};

pub type Classification = Vec<Vec<String>>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub classification: Classification,
    pub root_folder_name: String,
}

impl ::std::default::Default for AppConfig {
    fn default() -> Self {
        let default_classification: Classification = vec![
            vec!["ATTACK", "DECAY", "SUSTAIN", "RELEASE"],
            vec!["DARK", "NEUTRAL", "BRIGHT"],
            vec!["DISCO", "ELECTRO", "HOUSE", "ROCK", "TECHNO", "TRANCE"],
        ]
        .iter()
        .map(|row| row.iter().map(|&class| class.to_string()).collect())
        .collect();

        Self {
            classification: default_classification,
            root_folder_name: "ROOT".to_string()
        }
    }
}
