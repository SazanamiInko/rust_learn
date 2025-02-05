//////////////////////////////////
/// 
/// ねこ構造体
/// 
/// ///////////////////////////////

use crate::logics::animal::traits::Animal;

///ねこ構造体
pub struct Cat{}

///実装
impl Animal for Cat
{
  ///コンストラクタ
  fn new(_name:&str)->Self
  {
      return Cat{};
  }

    ///歩く
    fn walk(&self)
    {
        println!("チョコチョコ");
    }

    fn cry(&self)
    {
        println!("にゃーにゃー");
    }
}
