/////////////////////////////////
///
/// Rust標準入力
/// 
//////////////////////////////////

//利用標準ライブラリ
use std::io;

/*
メイン庵数
*/
fn main() {
  //入力を格納する変数
   let mut buf:String =String::new();

  //入力を促すメッセージ
  println!("何かキーを押してください");
  //入力受付
  io::stdin().read_line(&mut buf);
  //入力したメッセージを表示
  println!("入力された値:{}",buf);
}
