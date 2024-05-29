///////////////////////////////////////////////////////////
/// 
/// ファイル読み込み
/// 
/// ////////////////////////////////////////////////////////
use std::fs::File;
use std::io::prelude::*;

///基盤のパス
fn get_base_path()->String
{
    return String::from(r"C:\Users\Public\Documents\Data\Rust\work\pg0121\");
}

///indexページのパス
fn get_index_page_path()->String
{
    return String::from("index.html"); 
}

///インデックスページを表示
pub fn get_index_page()->String
{
    let base=get_base_path();
    let sub=get_index_page_path();
    let path=format!("{}{}",base,sub);
    let mut html=String::from("");  
    let mut file =File::open(path).expect("ファイルが開けませんでした");
    file.read_to_string(&mut html).expect("ファイルが読めませんでした");
    return html;
}