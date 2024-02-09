////////////////////////////////
///
///コマンドライン入力
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


    //結果出力
    println!("コマンド引算の数は{}個です。",c_count);
}
