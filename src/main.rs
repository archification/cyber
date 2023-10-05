extern crate solarized;

mod uninstall;
pub mod common;
mod unarchive;

use solarized::{
    print_colored, print_fancy, clear,
    VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA, WHITE, GREY,
    BOLD, UNDERLINED, ITALIC,
    PrintMode::{NewLine, SameLine},
};
use uninstall::{uninstall_mod};
use common::{load_config_from_file};
use unarchive::unarchive;

fn main() {
    let config = load_config_from_file("config.yaml").expect("Failed to load config");
    intro();
    loop {
        match user_menu() {
            1 => unarchive(&config),
            2 => uninstall_mod(),
            3 => break,
            _ => println!("Invalid option"),
        }
    }
}

fn user_menu() -> usize {
    println!("Choose an option:");
    println!("1. Install new mods");
    println!("2. Uninstall mods");
    println!("3. Exit");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or(0)
}

fn intro() {
    clear();
    print_fancy(&[
        ("R", VIOLET, vec![]),
        ("a", BLUE, vec![]),
        ("i", CYAN, vec![]),
        ("n", GREEN, vec![]),
        ("b", YELLOW, vec![]),
        ("o", ORANGE, vec![]),
        ("w", RED, vec![]),
        ("s", MAGENTA, vec![]),
    ], NewLine);
    print_fancy(&[
        ("Hello", WHITE, vec![BOLD]),
        ("World", GREY, vec![UNDERLINED, ITALIC])
    ], SameLine);
    print_colored(
        &["White ", "Grey"],
        &[WHITE, GREY],
        NewLine
    );
}
