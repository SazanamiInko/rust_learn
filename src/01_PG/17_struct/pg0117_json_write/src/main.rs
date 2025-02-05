///////////////////////////////////////
/// 
/// メイン関数
/// 
/// ///////////////////////////////////
mod mesure;

use std::env;

///メイン関数
fn main() {

   const PRESURE_POS:usize=1;
   const TEMP_POS:usize=2;

   let args: Vec<String> = env::args().collect();
   let  c_count=args.len()-1;

   if c_count!=2
   {
      println!("引数を2個指定してください。");
      return;
   }

   let presure=args[PRESURE_POS].parse().unwrap();
   let temp=args[TEMP_POS].parse().unwrap();
   let record=mesure::record::from(&presure, &temp);
   _=mesure::file_io::write(&record);

}
