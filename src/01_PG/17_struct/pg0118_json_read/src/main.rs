///////////////////////////////////////
/// 
/// Json読み出し
/// 
/// ///////////////////////////////////
mod mesure;
use std::env;

///メイン関数
fn main() {

   const MODE_POS:usize=1;
   const PRESURE_POS:usize=2;
   const TEMP_POS:usize=3;

   let args: Vec<String> = env::args().collect();
   let  c_count=args.len()-1;

   if c_count==0
   {
        println!("引数を指定してください");
        return;
   }
   
   if args[MODE_POS]=="W"
    {
        println!("書き込みモード");
        let presure=args[PRESURE_POS].parse().unwrap();
        let temp=args[TEMP_POS].parse().unwrap();
        let record=mesure::record::from(&presure, &temp);
        _=mesure::file_io::write(&record);
        println!("書き込みました");
    }
    else
    {
        println!("読み込みモード");
        mesure::file_io::read(); 
        println!("終了しました");  
    }
}
