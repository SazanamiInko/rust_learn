//////////////////////////////////////
/// 
/// SQLiteの検索
/// 
/// ///////////////////////////////////
mod logics;
use std::env;
use crate::logics::sale_api;

///主関数
fn main() 
{
    //引数チェック
    const ARG_LENGTH:usize=2;
    const ARG_DATE_POS:usize=1;
    const ARG_TAKE_ON_POS:usize=2;

     let args: Vec<String> = env::args().collect();
     let c_count=args.len()-1;
   
      //引数チェック
      if c_count!=ARG_LENGTH
      {
           println!("引数の数が違います");
           return;
      }

    let date=args[ARG_DATE_POS].clone();
    let take_on:i32=args[ARG_TAKE_ON_POS].parse().expect("乗駅は数字で指定してください");
    
    let result=sale_api::get_by_from(&date, take_on);
    
    match result
    {
        Ok(records)=>
        {
            println!("{}件のデータがあります",records.len());

            for record in records
            {
                println!("{},{},{}",record.id,record.take_off,record.time);
            }
        }
        Err(e)=>
        {
            println!("{:?}",e);
            println!("売上データの検索に失敗しました");
            return;
        }
    }


    println!("処理が終わりました。");
}
