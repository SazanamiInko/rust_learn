//////////////////////////////////
/// 
/// SQLite関連API
/// 
/// //////////////////////////////
use std::error::Error;
use rusqlite::Connection;
use rusqlite:: Result;

///GBファイルを作成
pub fn create_dbfile(filename:&String)->Result<(),Box<dyn Error>>
{
  
    let query=String::from(r"CREATE TABLE IF NOT EXISTS users (
             id INTEGER PRIMARY KEY,
             name TEXT NOT NULL,
             address TEXT NOT NULL,
             phone TEXT NOT NULL,
             entry TEXT NOT NULL,
             cancel TEXT NOT NULL,
             age INTEGER NOT NULL
         )");

     // SQLiteデータベースファイルに接続（なければ作成）
     let conn = Connection::open(filename)?;

     // テーブルの作成
     conn.execute(query.as_str(),[])?;
         
    return Ok(());
}
