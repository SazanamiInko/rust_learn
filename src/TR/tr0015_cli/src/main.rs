//////////////////////////////////////
/// 
/// ライブラリの利用
/// 
//////////////////////////////////////

use std::{default, env};
use tr0015_lib as lib;

/// メイン関数
fn main() {

  //定数
  //引数の数
  const ARGS_NUMBER:usize=2;

  //引数位置
  const POS_MODE:usize=1;
  const POS_VALUE:usize=2;

  //モード
  const P1_MODE:u32=1;
  const P3_MODE:u32=3;

  //引数の数をチェック
  let args: Vec<String> = env::args().collect();
  let arg_count=args.len()-1;

  let result:String;
 
  if arg_count!=ARGS_NUMBER
  {
    println!("引数の数が違います");
    return;
  }

 //第1引数のチェック
  let arg_mode:u32=args[POS_MODE].parse()
  .expect("モードを整数に変換できました");

  if arg_mode!=P1_MODE
  {
    if arg_mode!=P3_MODE
    {
        println!("モードの指定に誤りがあります");
        return;
    }
  }
 
  //第2引数のチェック
  let arg_target=args[POS_VALUE]
                 .parse()
                 .expect("第2引数が数値に変換できませんでした");

//ライブラリ関数の呼び出し
match arg_mode {
  P1_MODE=>{
    result=lib::round_p1(arg_target)
  }
  P3_MODE=>{
    result=lib::round_p3(arg_target)
  }
  default=>{return}
}

//結果の表示
println!("{}",result); 

}
