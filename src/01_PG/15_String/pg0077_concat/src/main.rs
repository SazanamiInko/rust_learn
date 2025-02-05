////////////////////////////
/// 
/// 文字列の結合
/// 
/// ////////////////////////
use chrono::Local;

fn main() {
    let date=Local::now();
    let fmt="%Y%m%d";
    let now_str=format!("{}",date.format(fmt)).to_string();
    let filename=now_str+".png"; 

    println!("{}",filename);
}