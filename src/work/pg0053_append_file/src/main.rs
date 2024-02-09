////////////////////////
/// 
/// ファイル追記
/// 
/// ////////////////////

//クレート
extern crate chrono;


//標準ライブラリ
use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::Local;


///メイン関数
fn main()->std::io::Result<()>{
    let date=Local::now();
    let path=r"C:\Users\Public\Documents\Data\log.txt".to_string();

    let mut file =OpenOptions::new()
    .append(true)
    .open(path)?;

    _=file.write(date.to_string().as_bytes());
    _=file.write("\n".as_bytes());
Ok(())
}
