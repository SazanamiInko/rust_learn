///////////////////////////////
/// 
/// DBの更新(マスタカードの論理削除)
/// 
/////////////////////////////////
mod logics;
use std::env;

///主関数
fn main() {

    const ARG_LENGTH:usize=2;
    const ARG_DELETE_CARD_POS:usize=1;
    const ARG_APPLICANT_CARD_POS:usize=2;

     let args: Vec<String> = env::args().collect();
     let c_count=args.len()-1;
   
      //引数チェック
      if c_count!=ARG_LENGTH
      {
           println!("引数の数が違います");
           return;
      }
   
      //カードの削除
      let delete_ferica=args[ARG_DELETE_CARD_POS].as_str();

      let applicant_ferica=args[ARG_APPLICANT_CARD_POS].as_str();
     match logics::ferica_api::delete_ferica(delete_ferica, applicant_ferica)
     {
        Ok(())=>{println!("カード情報を削除しました");},
        Err(e)=>
        {
             println!("{}",e);
             println!("カード情報の削除に失敗しました");
        }
     }
   
}
