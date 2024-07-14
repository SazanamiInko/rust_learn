////////////////////////////////////////////
/// 
/// DBの登録
/// 
/// /////////////////////////////////////////
mod logics;
use std::env;

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
   let new_ferica=args[ARG_ADD_CARD_POS].as_str();
   let adder_ferica=args[ARG_ADDER_CARD_POS].as_str();
  match logics::ferica_api::add_ferica(new_ferica, adder_ferica)
  {
     Ok(())=>{println!("カード情報を登録しました");},
     Err(e)=>
     {
          println!("{}",e);
          println!("カード情報の登録に失敗しました");
     }
  }
   
}
