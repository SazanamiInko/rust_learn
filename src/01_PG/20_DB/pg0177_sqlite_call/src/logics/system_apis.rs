/////////////////////////////
/// 
/// システム情報API
/// 
/// /////////////////////////
use rusqlite::{Connection, Result};

///SQLiteのバージョン情報取得
pub fn get_db_version()->Result<String,Box<dyn std::error::Error>>
{
    let query="SELECT sqlite_version()";

    let conn = Connection::open_in_memory()?;
    let mut stmt = conn.prepare(query)?;
    let version: String = stmt.query_row([], |row| row.get(0))?;
    
   Ok(version)
}