////////////////////////////
/// 
/// ファイル削除
/// 
/// ////////////////////////
use std::fs;

//メイン関数
fn main() {
    let path=r"C:\Users\Public\Documents\Data\Rust\work\pg0096\Operation.txt";

    fs::remove_file(path).expect("失敗しました。");
    println!("起動したから削除したよ");
}
