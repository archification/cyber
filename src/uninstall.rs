use std::fs::{self, File};
use std::path::Path;
use serde_json;
use crate::common::{ModRecord, load_mod_records};
/*
use solarized::{
    print_colored, print_fancy, clear,
    VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA, WHITE, GREY,
    BOLD, UNDERLINED, ITALIC,
    PrintMode::{NewLine, SameLine},
};
*/

fn update_mod_records(path: &str, records: &Vec<ModRecord>) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;
    serde_json::to_writer(file, records)?;
    Ok(())
}

pub fn uninstall_mod() {
    let mut records = load_mod_records("mod_records.json").expect("Failed to load mod records");
    if records.is_empty() {
        println!("No mods installed.");
        return;
    }
    println!("Choose a mod to uninstall:");
    for (index, record) in records.iter().enumerate() {
        println!("{}. {}", index + 1, record.source_archive);
    }
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse::<usize>() {
        Ok(choice) if choice >= 1 && choice <= records.len() => {
            let selected_record = &records[choice - 1];
            let mut paths_to_remove: Vec<_> = selected_record.installed_files.iter().collect();
            paths_to_remove.sort_by(|a, b| b.cmp(a));
            for file_path in paths_to_remove {
                if Path::new(file_path).is_dir() {
                    if let Err(e) = fs::remove_dir(file_path) {
                        eprintln!("Error removing directory {}: {}", file_path, e);
                    }
                } else if let Err(e) = fs::remove_file(file_path) {
                    eprintln!("Error removing file {}: {}", file_path, e);
                }
            }
            records.remove(choice - 1);
            update_mod_records("mod_records.json", &records).expect("Failed to update mod records");
            println!("Mod uninstalled successfully!");
        },
        _ => {
            println!("Invalid choice");
        }
    }
}
