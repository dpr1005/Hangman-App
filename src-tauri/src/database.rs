use rusqlite::{Connection, Result};

use crate::structs::Word;

pub fn insert_record(record: Word) -> Result<()> {
    let size = record.word.len() as i32;
    let conn: Connection = Connection::open("database.sqlite").unwrap();
   
    conn.execute(
        "INSERT INTO words (word, language, type, group_, size) VALUES (?1, ?2, ?3, ?4, ?5)",
        &[&record.word, &record.language, &record.type_, &record.group, &size.to_string()],
    )?;
    print!("DONE");
    Ok(())
}

pub fn remove_record(record: String) -> Result<()> {
    let conn: Connection = Connection::open("database.sqlite").unwrap();

    conn.execute(
        "DELETE FROM words WHERE word = ?1",
        &[&record],
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
    let query;
    let mut stmt;
    let mut rows;

    if type_ == "all" && group_ != "all" {
        query = "SELECT DISTINCT size FROM words WHERE group_ = $1";
        stmt = conn.prepare(query).unwrap();
        rows = stmt.query(&[&group_])?;
    }
    else if group_ == "all" && type_ != "all" {
        query = "SELECT DISTINCT size FROM words WHERE type = $1";
        stmt = conn.prepare(query).unwrap();
        rows = stmt.query(&[&type_])?;
    }
    else if type_ != "all" && group_ != "all" {
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


pub fn get_words(type_: String, group_: String, length: String) -> Result<Vec<String>> {
    let conn: Connection = Connection::open("database.sqlite").unwrap();
    let mut query: String = "".to_owned();
    let mut stmt;
    let mut rows;
    let mut size: String = " size = ".to_owned();
    let mut use_size: bool = false;

    if length.bytes().all(|c| c.is_ascii_digit()) {
        use_size = true;
        size.push_str(length.as_str());
    }

    if type_ == "all" && group_ != "all" {
        query.push_str("SELECT word FROM words WHERE group_ = $1");
        if use_size {
            query.push_str(" AND ");
            query.push_str(&size);
        }
        stmt = conn.prepare(&query).unwrap();
        rows = stmt.query(&[&group_])?;
    }
    else if group_ == "all" && type_ != "all" {
        query.push_str("SELECT word FROM words WHERE type = $1");
        if use_size {
            query.push_str(" AND ");
            query.push_str(&size);
        }
        stmt = conn.prepare(&query).unwrap();
        rows = stmt.query(&[&type_])?;
    }
    else if type_ != "all" && group_ != "all" {
        query.push_str("SELECT word FROM words WHERE type = $1 AND group_ = $2");
        if use_size {
            query.push_str(" AND ");
            query.push_str(&size);
        }
        stmt = conn.prepare(&query).unwrap();
        rows = stmt.query(&[&type_, &group_])?;
    } else {
        query.push_str("SELECT word FROM words");
        if use_size {
            query.push_str(" WHERE ");
            query.push_str(&size);
        }
        stmt = conn.prepare(&query).unwrap();
        rows = stmt.query([])?;
    }

    let mut words = Vec::new();
    while let Some(row) = rows.next()? {
        words.push(row.get(0)?);
    }

    Ok(words)
}