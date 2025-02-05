////////////////////////////
/// 
/// パスの結合
/// 
/// ////////////////////////
use std::path::Path;

//メイン関数
fn main() {
   let base_path=Path::new(r"C:\Users\Public\Documents\Data\Rust\work\pg0081\");
   let misyori=base_path.join("未処理");
   let basepath_str=format!("{:?}",base_path);
   let misyori_str=format!("{:?}",misyori);


   println!("基本フォルダ:{}",basepath_str);
   println!("未処理のフォルダ:{:?}",misyori_str);
}
