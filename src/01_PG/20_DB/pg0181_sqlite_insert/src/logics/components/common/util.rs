//////////////////////////
/// 
/// ユーティリティ
/// 
///////////////////////////
use chrono::Local;

///ファイル名の取得
pub fn get_file_name()->String
{
    let path=String::from(r"C:\Users\Public\Documents\data\Rust\pg0181");
    let today = Local::now().format("%Y%m%d").to_string();
    let filename=format!(r"{}\{}.db",path,today);
    return filename;
}