////////////////////////////
/// 
/// ファイルコピー
/// 
/// ////////////////////////
use std::fs;

fn main() {
    let from_path=r"C:\Users\Public\Documents\Data\Rust\work\pg0081\未処理\処理前.txt";
    let to_path=r"C:\Users\Public\Documents\Data\Rust\work\pg0081\処理中\処理待.txt";

    match fs::copy(from_path,to_path)
    {
        Ok(_)=>{println!{"{}","ファイルコピーしました"};}
        Err(_e)=>{println!{"{}","ファイルコピーに失敗しました"};}
    }
}
