use rusqlite::{Connection, Result};

use crate::structs::Word;
use super::DB_PATH;

pub fn insert_word(record: Word) -> Result<()> {
    let size = record.word.len() as i32;

    let langs_in_db = get_unique_langs().unwrap();
    let types_in_db = get_unique_types(record.language.clone()).unwrap();

    if !langs_in_db.contains(&record.language) {        
        insert_lang(record.language.clone()).unwrap();
    }

    if !types_in_db.contains(&record.type_) {
        insert_type(record.type_.clone(), record.language.clone()).unwrap();
    }

    let conn: Connection = Connection::open(DB_PATH).unwrap();

    match conn.execute("INSERT INTO words (word, language, type, group_, size) VALUES (?1, ?2, ?3, ?4, ?5)", 
    &[&record.word, &record.language, &record.type_, &record.group, &size.to_string()]) {
        Ok(_updated) => (),
        Err(err) => println!("Adding new word failed: {:?}", err),
    }

   close_connection(conn);
    
    Ok(())
}

pub fn remove_word_(word: String, mut lang: String) -> Result<()> {
    let conn: Connection = Connection::open(DB_PATH).unwrap();

    lang = lang.to_uppercase();

    conn.execute(
        "DELETE FROM words WHERE word = ?1 AND language = ?2",
        &[&word, &lang],
    )?;

    close_connection(conn);

    Ok(())
}

pub fn get_unique_lengths(type_: String, group_: String, mut lang: String) -> Result<Vec<i32>> {
    let conn: Connection = Connection::open(DB_PATH).unwrap();
    let query;
    let mut stmt;
    let mut rows;

    lang = lang.to_uppercase();

    if type_ == "all" && group_ != "all" {
        query = "SELECT DISTINCT size FROM words WHERE group_ = $1 AND language = $2";
        stmt = conn.prepare(query).unwrap();
        rows = stmt.query(&[&group_, &lang])?;
    }
    else if group_ == "all" && type_ != "all" {
        query = "SELECT DISTINCT size FROM words WHERE type = $1 AND language = $2";
        stmt = conn.prepare(query).unwrap();
        rows = stmt.query(&[&type_, &lang])?;
    }
    else if type_ != "all" && group_ != "all" {
        query = "SELECT DISTINCT size FROM words WHERE type = $1 AND group_ = $2 AND language = $3";
        stmt = conn.prepare(query).unwrap();
        rows = stmt.query(&[&type_, &group_, &lang])?;
    } else {
        query = "SELECT DISTINCT size FROM words WHERE language = $1";
        stmt = conn.prepare(query).unwrap();
        rows = stmt.query(&[&lang])?;
    }

    let mut lengths = Vec::new();
    while let Some(row) = rows.next()? {
        lengths.push(row.get(0)?);
    }

    Ok(lengths)
}


pub fn get_words(type_: String, group_: String, length: String, mut lang: String) -> Result<Vec<String>> {
    let conn: Connection = Connection::open(DB_PATH).unwrap();
    let mut query: String = "".to_owned();
    let mut stmt;
    let mut rows;
    let mut size: String = " AND size = ".to_owned();
    let mut use_size: bool = false;

    lang = lang.to_uppercase();

    if length.bytes().all(|c| c.is_ascii_digit()) {
        use_size = true;
        size.push_str(length.as_str());
    }

    if type_ == "all" && group_ != "all" {
        query.push_str("SELECT word FROM words WHERE group_ = $1 AND language = $2");
        if use_size {
            query.push_str(&size);
        }
        stmt = conn.prepare(&query).unwrap();
        rows = stmt.query(&[&group_, &lang])?;
    }
    else if group_ == "all" && type_ != "all" {
        query.push_str("SELECT word FROM words WHERE type = $1 AND language = $2");
        if use_size {
            query.push_str(&size);
        }
        stmt = conn.prepare(&query).unwrap();
        rows = stmt.query(&[&type_, &lang])?;
    }
    else if type_ != "all" && group_ != "all" {
        query.push_str("SELECT word FROM words WHERE type = $1 AND group_ = $2 AND language = $3");
        if use_size {
            query.push_str(&size);
        }
        stmt = conn.prepare(&query).unwrap();
        rows = stmt.query(&[&type_, &group_, &lang])?;
    } else {
        query.push_str("SELECT word FROM words WHERE language = $1");
        if use_size {
            query.push_str(&size);
        }
        stmt = conn.prepare(&query).unwrap();
        rows = stmt.query(&[&lang])?;
    }

    let mut words = Vec::new();
    while let Some(row) = rows.next()? {
        words.push(row.get(0)?);
    }

    Ok(words)
}

pub fn get_unique_types(mut lang: String) -> Result<Vec<String>> {
    let conn: Connection = Connection::open(DB_PATH).unwrap();

    lang = lang.to_uppercase();

    let mut stmt = conn.prepare("SELECT DISTINCT type FROM words WHERE language = $1").unwrap();
    let mut rows = stmt.query(&[&lang])?;

    let mut types = Vec::new();
    while let Some(row) = rows.next()? {
        types.push(row.get(0)?);
    }

    Ok(types)
}

pub fn get_unique_langs() -> Result<Vec<String>> {
    let conn: Connection = Connection::open(DB_PATH).unwrap();

    let mut stmt = conn.prepare("SELECT DISTINCT language FROM languages").unwrap();
    let mut rows = stmt.query([])?;

    let mut languages = Vec::new();
    while let Some(row) = rows.next()? {
        languages.push(row.get(0)?);
    }

    Ok(languages)
}

pub fn get_unique_groups(mut lang: String) -> Result<Vec<String>> {
    let conn: Connection = Connection::open(DB_PATH).unwrap();

    lang = lang.to_uppercase();

    let mut stmt = conn.prepare("SELECT DISTINCT group_ FROM words WHERE language = $1").unwrap();
    let mut rows = stmt.query(&[&lang])?;

    let mut groups = Vec::new();
    while let Some(row) = rows.next()? {
        groups.push(row.get(0)?);
    }

    Ok(groups)
}

fn insert_type(type_: String, mut lang: String) -> Result<(), ()> {
    let conn: Connection = Connection::open(DB_PATH).unwrap();

    lang = lang.to_uppercase();

    match conn.execute("INSERT INTO types (type, language) VALUES (?1, ?2)", 
        &[&type_, &lang]) {
            Ok(_updated) => (),
            Err(err) => println!("update failed: {:?}", err),
        }

    close_connection(conn);

    Ok(())
}

fn insert_lang(mut lang: String) -> Result<(), ()> {
    let conn: Connection = Connection::open(DB_PATH).unwrap();

    lang = lang.to_uppercase();
    
     match conn.execute("INSERT INTO languages (language) VALUES (?1)", 
        &[&lang]) {
            Ok(_updated) => (),
            Err(err) => println!("update failed: {:?}", err),
        }

    close_connection(conn);

    Ok(())
}

pub fn remove_lang(mut lang: String) -> Result<()> {
    let conn: Connection = Connection::open(DB_PATH).unwrap();

    lang = lang.to_uppercase();

    let mut stmt = conn.prepare("DELETE FROM words WHERE language = $1").unwrap();
    stmt.execute(&[&lang])?;

    stmt = conn.prepare("DELETE FROM types WHERE language = $1").unwrap();
    stmt.execute(&[&lang])?;

    stmt = conn.prepare("DELETE FROM languages WHERE language = $1").unwrap();
    stmt.execute(&[&lang])?;

    Ok(())
}

pub fn remove_type_(type_: String, mut lang: String) -> Result<()> {
    let conn: Connection = Connection::open(DB_PATH).unwrap();

    lang = lang.to_uppercase();

    let mut stmt = conn.prepare("DELETE FROM words WHERE type = $1 AND language = $2").unwrap();
    stmt.execute(&[&type_, &lang])?;

    stmt = conn.prepare("DELETE FROM types WHERE type = $1 AND language = $2").unwrap();
    stmt.execute(&[&type_, &lang])?;

    Ok(())
}

pub fn remove_group_(group_: String, mut lang: String) -> Result<()> {
    let conn: Connection = Connection::open(DB_PATH).unwrap();

    lang = lang.to_uppercase();

    let mut stmt = conn.prepare("DELETE FROM words WHERE group_ = $1 AND language = $2").unwrap();
    stmt.execute(&[&group_, &lang])?;

    Ok(())
}

fn close_connection(conn: Connection) {
    match conn.close() {
        Ok(()) => (),
        Err(err) => println!("Connection close failed: {:?}", err),
    }
}
