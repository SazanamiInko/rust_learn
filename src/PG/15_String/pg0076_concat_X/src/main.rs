////////////////////////////
/// 
/// 文字列の結合（ビルド失敗）
/// 
/// ////////////////////////
use chrono::Local;

fn main() {
    let date=Local::now();
    let fmt="%Y%m%d";
    let now_str=date.format(fmt);
    let filename=now_str+".png";

    println!("{}",filename);
}
