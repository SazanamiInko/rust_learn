////////////////////////////////////////////
/// 
/// DBの登録
/// 
/// /////////////////////////////////////////
mod logics;
use std::env;
use crate::logics::ferica_api;

///メイン関数
fn main() {

    const ARG_LENGTH:usize=2;
    const ARG_ADD_CARD_POS:usize=1;
    const ARG_ADDER_CARD_POS:usize=2;


   //コマンド引数を取得
   let mut c_count=0;
   //とりあえずvectorは置いといていて
   let args: Vec<String> = env::args().collect();
   c_count=args.len()-1;

   //引数チェック
   if c_count!=ARG_LENGTH
   {
        println!("引数の数が違います");
        return;
   }

   //カードのチェック
   let add_card=args[ARG_ADD_CARD_POS].as_str();
   let adder_card=args[ARG_ADDER_CARD_POS].as_str();
   match ferica_api::check(add_card, adder_card)
   {
        Ok(())=>{},
        Err(e)=>
        {
            println!("{}",e);
            println!("エラーがありましたので終了します。");
            return;
        }  
   }

   println!("カード情報を登録しました");
}
