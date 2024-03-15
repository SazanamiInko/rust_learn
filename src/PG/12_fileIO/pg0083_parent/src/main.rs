////////////////////////////
/// 
/// 親フォルダ取得
/// 
/// ////////////////////////
use std::path::Path;

//メイン関数
fn main() {
    let now_path=Path::new(r"C:\Users\Public\Documents\Data\Rust\work\pg0081\未処理");
    let parent_path=now_path.parent().unwrap();

   let now_path_str=now_path.to_string_lossy();
   let parent_str=parent_path.to_string_lossy();

   println!("現在:{}",now_path_str);
   println!("親:{}",parent_str);
}
