////////////////////////////
/// 
/// if_let関数
/// 
/// ////////////////////////

use std::fs;

//メイン関数
fn main() {
    let from_path=String::from(r"C:\Users\Public\Documents\Data\Rust\work\pg0081\未処理\処理前.txt");
    let to_path=String::from(r"C:\Users\Public\Documents\Data\Rust\work\pg0081\処理済\処理済.txt");
   
    println!("{}", move_file(from_path, to_path));

}

//ファイル移動する関数
fn move_file(from:String,to:String)->String
{
    let move_result= fs::rename(from,to);
    if let Ok(_value) =move_result
    {
       return String::from("OK");
    }
    
    if let Err(e)=move_result
    {
        return format!("{}",e);
    }

    return String::from("unexpected error");

}