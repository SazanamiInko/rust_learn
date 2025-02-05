//////////////////////////////
/// 
///Webクライアント
/// 
/// //////////////////////////
mod logics;
use std::env;
use crate::logics::postal_api;

///主関数
fn main() {
 
    const ARG_LENGTH:usize=1;
    const ARG_ZIPCODE_POS:usize=1;

     let args: Vec<String> = env::args().collect();
     let c_count=args.len()-1;
   
      //引数チェック
      if c_count!=ARG_LENGTH
      {
           println!("引数の数が違います");
           return;
      }

      let zip_code=args[ARG_ZIPCODE_POS].as_str();

      let result=postal_api::get_address(zip_code);

      match result {

        Ok(response)=>{println!("{}",response);}
        Err(e)=>
        {
            println!("{}",e);
            println!("郵便番号から住所の取得に失敗しました");
        }
          
      }
    }
