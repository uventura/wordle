use rusqlite::{Connection, Result, params};


pub fn update_palavra_do_dia(conn: &Connection, id: i32, nova_palavra: &str) -> Result<()> {
    conn.execute(
        "UPDATE wordle SET PalavradoDia = ?1 WHERE id = ?2",
        params![nova_palavra, id],
    )?;
    Ok(())
}


pub fn insert_palavra_do_dia(conn: &Connection, palavra: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO wordle (PalavradoDia) VALUES (?1)",
        params![palavra],
    )?;
    Ok(())
}

pub fn get_palavra_do_dia(conn: &Connection, id: i32) -> Result<String> {
    conn.query_row(
        "SELECT PalavradoDia FROM wordle WHERE id = ?1",
        params![id],
        |row| row.get(0),
    )
}
