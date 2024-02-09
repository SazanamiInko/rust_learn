///////////////////////////////
/// 
/// 乱数
/// 
/// ///////////////////////////

use rand::{Rng,SeedableRng};
///メイン関数
fn main() {
   let mut gen=rand::thread_rng();

   for cnt in 1..11
   {
      println!("{:>2} 回目 - {}",cnt,gen.gen_range(1..6));
   }
}
