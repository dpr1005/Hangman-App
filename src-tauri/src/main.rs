#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod database;
pub mod structs;
use database::{insert_record, remove_record, unique_types, unique_groups, unique_lengths, get_words};
use structs::Word;

use rusqlite::{Result};

fn main() -> Result<()> {    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            kill_app, find_uniques, find_groups, find_lengths, generate_word, add_word, remove_word
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

        Ok(())
}

#[tauri::command]
fn kill_app() {
    panic!("Killing app...");
}

#[tauri::command]
fn find_uniques() -> Vec<String>{
    return unique_types().unwrap();
}

#[tauri::command]
fn find_groups () -> Vec<String>{
    return unique_groups().unwrap();
}

#[tauri::command]
fn find_lengths(word_type: String, group: String) -> Vec<i32>{
    return unique_lengths(word_type, group).unwrap();
}

#[tauri::command]
fn generate_word(word_type: String, group: String, length: String) -> Vec<String>{
    return get_words(word_type, group, length).unwrap();
}

#[tauri::command]
fn add_word(word: String, language: String, type_: String, group: String) {
    println!("Adding word: {}", word);
    let size = word.chars().count() as i32;
    insert_record(Word{
        word: word,
        language: language,
        type_: type_,
        group: group,
        size: size,
    }).unwrap();
}

#[tauri::command]
fn remove_word(word: String) {
    remove_record(word).unwrap();
}