////////////////////////////
/// 
/// ファイル移動
/// 
/// ////////////////////////
use std::fs;

fn main() {
    let from_path=r"C:\Users\Public\Documents\Data\Rust\work\pg0081\未処理\処理前.txt";
    let to_path=r"C:\Users\Public\Documents\Data\Rust\work\pg0081\処理済\処理済.txt";
    match fs::rename(from_path,to_path)
    {
        Ok(_)=>{println!{"{}","ファイル移動しました"};}
        Err(_e)=>{println!{"{}","ファイル移動に失敗しました"};}
    }
}
