#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod database;
pub mod structs;
use database::{insert_record, remove_record, unique_types, unique_groups, unique_lengths};
use structs::Word;

use rusqlite::{Connection, Result, types};

#[tauri::command]
fn kill_app() {
    println!("Killing app...");
    panic!("Killing app...");
}
fn main() -> Result<()> {
    let conn_mut = Connection::open("database.sqlite")?;
    let conn: Connection = Connection::open("database.sqlite").unwrap();
    //conn = Some(Connection::open("database.sqlite")?);

    // let mut stmt = conn.prepare("SELECT * FROM words")?;

    // let rows = stmt.query_map([], |row| {
    //     Ok(Word {
    //         id: row.get(0)?,
    //         word: row.get(1)?,
    //         language: row.get(2)?,
    //         type_: row.get(3)?,
    //         group: row.get(4)?,
    //         size: row.get(5)?,
    //     })
    // })?;

    // insert_record(&mut conn_mut,  Word {
    //     id: 1000,
    //     word: "hello".to_string(),
    //     language: "en".to_string(),
    //     type_: "noun".to_string(),
    //     group: "noun".to_string(),
    //     size: 5,
    // })?;

    // rows.for_each(|row| {
    //     println!("{:?}", row.unwrap());
    // });

    
    // https://rust-lang-nursery.github.io/rust-cookbook/database/sqlite.html#sqlite

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![kill_app, find_uniques, find_groups, find_lengths])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

        Ok(())
}

#[tauri::command]
fn find_uniques() -> Vec<String>{
    println!("{:?}", unique_types().unwrap());
    return unique_types().unwrap();
}

#[tauri::command]
fn find_groups () -> Vec<String>{
    println!("{:?}", unique_groups().unwrap());
    return unique_groups().unwrap();
}

#[tauri::command]
fn find_lengths(word_type: String, group: String) -> Vec<i32>{
    println!("{:?}", unique_lengths(word_type.clone(), group.clone()).unwrap());
    return unique_lengths(word_type, group).unwrap();
}
