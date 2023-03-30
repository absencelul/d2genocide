use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tinytemplate::TinyTemplate;

use crate::d2::draw::{Alignment, TextColor};

#[derive(Debug, Serialize, Deserialize)]
pub struct UiInfoConfig {
    pub x: i32,
    pub y: i32,
    pub color: TextColor,
    pub alignment: Alignment,
    pub font: u8,
    pub message: String,

    #[serde(skip_deserializing)]
    pub tt: String,
}

impl UiInfoConfig {
    fn create_tiny_template(&self) -> TinyTemplate {
        let mut template = TinyTemplate::new();
        template.add_template("message", &self.message).unwrap();
        template
    }

    pub fn render(&self, variables: &HashMap<&str, String>) -> String {
        let tt = self.create_tiny_template();
        let msg = tt.render("message", variables).unwrap();
        msg
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UiConfig {
    pub automap: Vec<UiInfoConfig>,
    pub screen: Vec<UiInfoConfig>,
    #[serde(skip)]
    pub file_name: String,
}

impl UiConfig {
    pub fn new(file_name: &str) -> Self {
        Self {
            file_name: file_name.to_string(),
            automap: Vec::new(),
            screen: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<(), serde_json::Error> {
        let file = std::fs::File::open(&self.file_name).expect("Unable to open file");
        let reader = std::io::BufReader::new(file);
        let config: UiConfig = serde_json::from_reader(reader)?;
        self.automap = config.automap;
        self.screen = config.screen;
        Ok(())
    }
}
