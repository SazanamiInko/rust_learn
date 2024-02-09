//////////////////////////////////
///
///else
/// 
//////////////////////////////////

//使用標準ライブラリ
use std::env;

// . メイン関数
fn main() {

    //入力
    let mut c_count=0;
    //とりあえずvectorは置いといていて
    let args: Vec<String> = env::args().collect();
    c_count=args.len()-1;

    if(c_count==2)
    {
        println!("処理します")
    }
    else
    {
        println!("引数が無効です")
    }
}
