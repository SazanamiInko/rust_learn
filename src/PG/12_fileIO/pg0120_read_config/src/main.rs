
/////////////////////////////////
///
/// 設定ファイルのIO
/// 
//////////////////////////////////
mod logics;
use std::env;

///メイン関数
fn main() {

    let args: Vec<String> = env::args().collect();
    let c_count=args.len()-1;

    let conf=logics::config::read_config()
                       .unwrap();

    if c_count>0{
        match conf.write_config() {
            Ok(())=>println!("設定ファイルの保存に成功しました"),
            Err(message)=>println!("{}",message),
        }
    }
    else
    {
        conf.display();
    }
    
}
