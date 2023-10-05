use std::fs::{self, File, DirBuilder, /*OpenOptions*/};
use std::path::Path;
use serde_json;
use zip::ZipArchive;
/*
use solarized::{
    print_colored, print_fancy, clear,
    VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA, WHITE, GREY,
    BOLD, UNDERLINED, ITALIC,
    PrintMode::{NewLine, SameLine},
};
*/
use crate::common::{load_mod_records, ModRecord, Config};

fn save_mod_record(record: &ModRecord, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut records = if Path::new(path).exists() {
        load_mod_records(path)?
    } else {
        Vec::new()
    };
    records.push(record.clone());
    let file = File::create(path)?;
    serde_json::to_writer(&file, &records)?;
    Ok(())
}

pub fn unarchive(config: &Config) {
    let dir_path = Path::new(&config.dir_path);
    let game_path = Path::new(&config.game_path);
    for entry in fs::read_dir(dir_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                let folder_name = path.file_stem().unwrap().to_str().unwrap();
                let dest_path = dir_path.join(folder_name);
                DirBuilder::new().recursive(true).create(&dest_path).unwrap();
                let mut mod_record = ModRecord {
                    source_archive: path.to_str().unwrap().to_string(),
                    installed_files: Vec::new(),
                };
                match ext.to_str().unwrap() {
                    "zip" => {
                        let file = fs::File::open(&path).unwrap();
                        let mut archive = ZipArchive::new(file).unwrap();
                        for i in 0..archive.len() {
                            let mut file = archive.by_index(i).unwrap();
                            let file_path = Path::new(file.name());
                            let game_outpath = if file_path.is_absolute() {
                                file_path.to_owned()
                            } else {
                                game_path.join(file_path)
                            };
                            mod_record.installed_files.push(game_outpath.to_str().unwrap().to_string());
                            if file.name().ends_with('/') {
                                DirBuilder::new().recursive(true).create(&game_outpath).unwrap();
                            } else {
                                let mut outfile = fs::File::create(&game_outpath).unwrap();
                                std::io::copy(&mut file, &mut outfile).unwrap();
                            }
                        }
                    },
                    _ => {}
                }
                println!("ModRecord: {:?}", mod_record);
                save_mod_record(&mod_record, "mod_records.json").expect("Failed to save mod record");
            }
        }
    }
}
