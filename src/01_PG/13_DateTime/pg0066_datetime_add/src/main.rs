/////////////////////////
///
/// 日時の加算
///
/////////////////////////

use chrono::{Local, Duration};

fn main() {
   let  local_now=Local::now();
   let post_now=local_now+Duration::minutes(3);

   println!("現在時刻は");
   println!("{}",local_now);
   println!("小池さんラーメンは");
   println!("{}",post_now);
   println!("にできるわよ");
   
}
