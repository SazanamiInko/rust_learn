////////////////////////////
/// 
/// SQLiteの挿入
/// 
/// /////////////////////////
mod logics;
use std::env;
use crate::logics::sale_api;

//主関数
fn main() {
//引数チェック
const ARG_LENGTH:usize=2;
    const ARG_TAKE_ON_POS:usize=1;
    const ARG_TAKE_OFF_POS:usize=2;

     let args: Vec<String> = env::args().collect();
     let c_count=args.len()-1;
   
      //引数チェック
      if c_count!=ARG_LENGTH
      {
           println!("引数の数が違います");
           return;
      }

    let take_on:i32=args[ARG_TAKE_ON_POS].parse().expect("乗駅は数字で指定してください");
    let take_off=args[ARG_TAKE_OFF_POS].parse().expect("降駅は数字で指定してください");

    //DBファイルの作成
    let create_reult=sale_api::create_dbfile();

    match create_reult {
        Ok(())=>{},
        Err(e)=>
        {
            println!("{:?}",e);
            println!("DBの作成に失敗しました");
            return;
        }
        
    }

    let add_result=sale_api::add_sale_record(take_on, take_off);

    match add_result {
        Ok(())=>
        {
            println!("売上を計上しました");
        }
        Err(e)=>{
            println!("{:?}",e);
            println!("売上を計上に失敗しました");
        }
        
    }

}
