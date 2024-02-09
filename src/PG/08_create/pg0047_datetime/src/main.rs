//////////////////////
/// 
/// 時刻
/// 
/// ///////////////////

//クレート
extern crate chrono;

//標準ライブラリ
use chrono::{Local, DateTime};


///メイン関数
fn main() {
  let date=Local::now();

  println!("{}",date);
}
