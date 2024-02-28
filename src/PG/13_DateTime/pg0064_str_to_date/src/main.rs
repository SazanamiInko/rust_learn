///////////////////////
///
/// 文字列->日付(標準)
///
////////////////////////
use chrono::DateTime;
fn main() {
    let datetime = "2018/12/07 19:31:28 +0900";
   let fmt="%Y/%m/%d %H:%M:%S %z";
   let wdate=DateTime::parse_from_str(datetime, fmt).unwrap();

   println!("{}",wdate ) ;
       
   }

