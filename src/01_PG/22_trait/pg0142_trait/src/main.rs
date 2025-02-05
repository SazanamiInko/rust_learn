///////////////////////////////////
/// 
/// トレイト
/// 
/// ///////////////////////////////
mod logics;

use logics::animal::Animal;
use crate::logics::models::cat;
use crate::logics::models::paracket;

///メイン関数
fn main() 
{
   let paracket=paracket::create("P");
 
   println!("");
   println!("インコちゃん");
   paracket.walk();
   paracket.cry();
   
   let cat=cat::create();

    println!("");
   println!("ねこちゃん");

   cat.walk();
   cat.cry();

}
