#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod database;
pub mod structs;
use database::insert_record;
use structs::Word;

use rusqlite::{Connection, Result};

#[tauri::command]
fn kill_app() {
    println!("Killing app...");
    panic!("Killing app...");
}

fn main() -> Result<()> {
    let mut conn_mut = Connection::open("database.sqlite")?;
    let conn = Connection::open("database.sqlite")?;

    let mut stmt = conn.prepare("SELECT * FROM words")?;

    let rows = stmt.query_map([], |row| {
        Ok(Word {
            id: row.get(0)?,
            word: row.get(1)?,
            language: row.get(2)?,
            type_: row.get(3)?,
            group: row.get(4)?,
            size: row.get(5)?,
        })
    })?;

    insert_record(&mut conn_mut,  Word {
        id: 1000,
        word: "hello".to_string(),
        language: "en".to_string(),
        type_: "noun".to_string(),
        group: "noun".to_string(),
        size: 5,
    })?;

    rows.for_each(|row| {
        println!("{:?}", row.unwrap());
    });


    // https://rust-lang-nursery.github.io/rust-cookbook/database/sqlite.html#sqlite

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![kill_app])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

        Ok(())
}
