use std::string;

use rusqlite::{Connection, Result, types, Statement, Rows};
use serde::de::value::Error;

use crate::structs::Word;

#[tauri::command]
pub fn insert_record(record: Word) -> Result<()> {
    let size = record.word.len() as i32;
    let conn: Connection = Connection::open("database.sqlite").unwrap();
   
    conn.execute(
        "INSERT INTO words (word, language, type, group_, size) VALUES (?1, ?2, ?3, ?4, ?5)",
        &[&record.word, &record.language, &record.type_, &record.group, &size.to_string()],
    )?;

    Ok(())
}

#[tauri::command]
pub fn remove_record() -> Result<()> {
    let conn: Connection = Connection::open("database.sqlite").unwrap();

    conn.execute(
        "DELETE FROM words WHERE id = $1",
        &[&1],
    )?;

    Ok(())
}

pub fn unique_types() -> Result<Vec<String>> {
    let conn: Connection = Connection::open("database.sqlite").unwrap();

    let mut stmt = conn.prepare("SELECT DISTINCT type FROM words").unwrap();
    let mut rows = stmt.query([])?;

    let mut types = Vec::new();
    while let Some(row) = rows.next()? {
        types.push(row.get(0)?);
    }

    Ok(types)
}

pub fn unique_groups() -> Result<Vec<String>> {
    let conn: Connection = Connection::open("database.sqlite").unwrap();

    let mut stmt = conn.prepare("SELECT DISTINCT group_ FROM words").unwrap();
    let mut rows = stmt.query([])?;

    let mut groups = Vec::new();
    while let Some(row) = rows.next()? {
        groups.push(row.get(0)?);
    }

    Ok(groups)
}

pub fn unique_lengths(type_: String, group_: String) -> Result<Vec<i32>> {
    let conn: Connection = Connection::open("database.sqlite").unwrap();
    let mut query = "";
    let mut stmt;
    let mut rows;

    if type_ == "random" && group_ != "random" {
        query = "SELECT DISTINCT size FROM words WHERE group_ = $1";
        stmt = conn.prepare(query).unwrap();
        rows = stmt.query(&[&group_])?;
    }
    else if group_ == "random" && type_ != "random" {
        query = "SELECT DISTINCT size FROM words WHERE type = $1";
        stmt = conn.prepare(query).unwrap();
        rows = stmt.query(&[&type_])?;
    }
    else if type_ != "random" && group_ != "random" {
        query = "SELECT DISTINCT size FROM words WHERE type = $1 AND group_ = $2";
        stmt = conn.prepare(query).unwrap();
        rows = stmt.query(&[&type_, &group_])?;
    } else {
        query = "SELECT DISTINCT size FROM words";
        stmt = conn.prepare(query).unwrap();
        rows = stmt.query([])?;
    }

    let mut lengths = Vec::new();
    while let Some(row) = rows.next()? {
        lengths.push(row.get(0)?);
    }

    Ok(lengths)
}
