///////////////////////
///
/// 文字列->日付(標準)
///
////////////////////////
use chrono::naive::NaiveDateTime;
fn main() {
    let datetime = "2018/12/07 19:31:28";
   let fmt="%Y/%m/%d %H:%M:%S";
   let wdate=NaiveDateTime::parse_from_str(datetime, fmt).unwrap();

   println!("{}",wdate ) ;
       
   }