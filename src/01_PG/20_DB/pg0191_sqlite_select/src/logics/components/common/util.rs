//////////////////////////
/// 
/// ユーティリティ
/// 
///////////////////////////
use chrono::Local;
use std::path::Path;

///ファイル名の取得
pub fn get_file_name()->String
{
    let path=String::from(r"C:\Users\Public\Documents\data\Rust\pg0181");
    let today = Local::now().format("%Y%m%d").to_string();
    let filename=format!(r"{}\{}.db",path,today);
    return filename;
}

//過去の売上データの取得用のパス取得
pub fn get_sale_data_path(date:&str)->String
{
    let path=String::from(r"C:\Users\Public\Documents\data\Rust\pg0181");
    let filename=format!(r"{}\{}.db",path,date);
    return filename;
}

///パス検査
pub fn is_exists_sale_data(date:&str)->bool
{
   let path= get_sale_data_path(date);

   return Path::new(&path).exists(); 
}