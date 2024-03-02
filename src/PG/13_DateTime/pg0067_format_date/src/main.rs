/////////////////////////
///
/// 時刻のフォーマット
///
/////////////////////////

use chrono::{Local, Duration};

fn main() {
   let  local_now=Local::now();
   let post_now=local_now+Duration::minutes(3);
   let fmt="%H時%M分";
   let now_str=local_now.format(fmt);
   let post_str=post_now.format(fmt);

  println!("現在時刻は");
   println!("{}",now_str);
   println!("小池さんラーメンは");
   println!("{}",post_str);
   println!("にできるわよ");
   
}
