////////////////////////////
/// 
/// タイムスタンプ
/// 
/// ////////////////////////
use std::fs;
use chrono::prelude::{DateTime, Utc, Datelike };

//メイン関数
fn main() {
let target_path=String::from(r"C:\Users\Public\Documents\Data\Rust\work\pg0081\処理済\処理済.txt");
    
   let meta= fs::metadata(target_path).unwrap();

   let created= meta.created().unwrap();
   let datetime_created_utc : DateTime<Utc>=created.into();
  
   println!("ファイルの作成日時は{}/{:02}/{:02}"
            ,datetime_created_utc.year()
            ,datetime_created_utc.month()
            ,datetime_created_utc.day() );
   
}
