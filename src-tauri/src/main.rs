#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rusqlite::{Connection, Result};

#[tauri::command]
fn kill_app() {
    println!("Killing app...");
    panic!("Killing app...");
}

fn main() -> Result<()> {
    let conn = Connection::open("database.sqlite")?;
    //conn.execute(
    //    "create table if not exists word (id integer primary key, name text not null, language text not null, type text not null)", []
    //)?;

    // https://rust-lang-nursery.github.io/rust-cookbook/database/sqlite.html#sqlite

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![kill_app])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

        Ok(())
}
