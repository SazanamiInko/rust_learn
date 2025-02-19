//////////////////////////////
/// 
/// AIサンプル：日付ー＞文字列復習
/// 
/// ///////////////////////////
use chrono::Local;


///主関数
fn main() {
    let today = Local::now().format("%Y%m%d").to_string();
    println!("{}", today); 
}
