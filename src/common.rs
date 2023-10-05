use std::fs::{File};
use std::io::{Read};
use serde::{Deserialize, Serialize};
use serde_json;
/*
use solarized::{
    print_colored, print_fancy, clear,
    VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA, WHITE, GREY,
    BOLD, UNDERLINED, ITALIC,
    PrintMode::{NewLine, SameLine},
};
*/

#[derive(Deserialize)]
pub struct Config {
    pub dir_path: String,
    pub game_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModRecord {
    pub source_archive: String,
    pub installed_files: Vec<String>,
}

pub fn load_config_from_file(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}

pub fn load_mod_records(path: &str) -> Result<Vec<ModRecord>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let records: Vec<ModRecord> = serde_json::from_reader(file)?;
    Ok(records)
}
