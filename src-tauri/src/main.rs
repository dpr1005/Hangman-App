#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod database;
pub mod structs;
use database::*;
use structs::Word;

use rusqlite::{Result};

static DB_PATH: &'static str = "../database/database.sqlite";

fn main() -> Result<()> {    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            kill_app, find_lengths, generate_word, 
            add_word, remove_word, get_languages, get_types, get_groups
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
fn find_lengths(word_type: String, group: String, language: String) -> Vec<i32>{
    return unique_lengths(word_type, group, language).unwrap();
}

#[tauri::command]
fn generate_word(word_type: String, group: String, length: String, language: String) -> Vec<String>{
    return get_words(word_type, group, length, language).unwrap();
}

#[tauri::command]
fn add_word(word: String, language: String, type_: String, group: String) {
    insert_word(Word{
        word: word,
        language: language,
        type_: type_,
        group: group,
    }).unwrap();
}

#[tauri::command]
fn remove_word(word: String) {
    vanish_word(word).unwrap();
}

#[tauri::command]
fn get_languages() -> Vec<String> {
    return get_unique_langs().unwrap();
}

#[tauri::command]
fn get_types(lang: String) -> Vec<String> {
    return get_unique_types(lang).unwrap();
}

#[tauri::command]
fn get_groups(lang: String) -> Vec<String> {
    return get_unique_groups(lang).unwrap();
}