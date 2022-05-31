


use rusqlite::{Connection, Result};

use crate::structs::Word;

pub fn insert_record(conn: &mut Connection, record: Word) -> Result<()> {
    
    let size = record.word.len() as i32;
    
    conn.execute(
        "INSERT INTO words (word, language, type, group_, size) VALUES (?1, ?2, ?3, ?4, ?5)",
        &[&record.word, &record.language, &record.type_, &record.group, &size.to_string()],
    )?;

    Ok(())
}

pub fn remove_record(conn: &mut Connection) -> Result<()> {
    conn.execute(
        "DELETE FROM words WHERE id = $1",
        &[&1],
    )?;

    Ok(())
}
