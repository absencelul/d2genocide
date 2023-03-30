use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChickenSettings {
    pub town_life: i32,
    pub town_mana: i32,
    pub exit_life: i32,
    pub exit_mana: i32,
    pub potion_life: i32,
    pub rejuv_life: i32,
    pub potion_mana: i32,
    pub rejuv_mana: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub chicken: ChickenSettings,

    #[serde(skip)]
    file_name: String,
}

impl Settings {
    pub fn new(file_name: &str) -> Self {
        Self {
            chicken: ChickenSettings {
                town_life: 0,
                town_mana: 0,
                exit_life: 0,
                exit_mana: 0,
                potion_life: 0,
                rejuv_life: 0,
                potion_mana: 0,
                rejuv_mana: 0,
            },
            file_name: file_name.to_string(),
        }
    }

    pub fn parse(&mut self) -> Result<(), serde_json::Error> {
        let file = std::fs::File::open(&self.file_name).expect("Unable to open file");
        let reader = std::io::BufReader::new(file);
        let config: Settings = serde_json::from_reader(reader)?;
        self.chicken = config.chicken;
        Ok(())
    }
}
