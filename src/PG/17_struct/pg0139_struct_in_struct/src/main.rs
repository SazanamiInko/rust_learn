////////////////////////////////
/// 
/// 構造体の入れ子
/// 
/// ////////////////////////////
mod logic;
use std::env;

///メイン関数
fn main() {
    
    let args: Vec<String> = env::args().collect();
    let count=args.len()-1;

    if count !=1
    {
        println!("引数が不正です.");
        return;
    }

    if args[1]=="W"
    {
        logic::setting::write();
        
    }
    else 
    {
        let setting=logic::setting::load();
        println!("{:?}",setting);
    }
}
