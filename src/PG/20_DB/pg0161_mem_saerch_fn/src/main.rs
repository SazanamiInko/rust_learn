///////////////////////////////////////
/// 
/// メイン関数
/// 
/// ///////////////////////////////////
mod logics;
use std::env;

use logics::discount_api;

///メイン関数
fn main() {
 
    
    //とりあえずvectorは置いといていて
    let args: Vec<String> = env::args().collect();
    let c_count=args.len()-1;

    if c_count!=1
    {
        println!("引数の数が不正です。");
        return;
    }

   let hour=args[1].trim()
                   .parse()
                   .expect("変換できませんでした。");

    let res_init=discount_api::init_discount();

    match res_init {
        Ok(())=>{},
        Err(e)=>
        {
            println!("{}",e);
            println!("初期化に失敗しました。");
            return;
        }
        
    }

    let dis=discount_api::get_discount(hour);

    match dis
    {
        Ok(result)=>
        {
            match result {
                None=>{println!("該当する割引がありませんでした");},
                Some(discount)=>{println!("{}",discount.name);}
                
            }
        },
        Err(e)=>
        {
            println!("{}",e);
            println!("割引情報が取得に失敗しました。");
        }

    }

 }
