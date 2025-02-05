/////////////////////////////////////////
/// 
/// 人間
/// 
/// //////////////////////////////////////
use crate::logics::animal::traits::Animal;

pub struct Human{}

///実装
impl Animal for Human
{
  ///コンストラクタ
  fn new(_name:&str)->Self
  {
      return Human{};
  }

    ///歩く
    fn walk(&self)
    {
        println!("こそこそ");
    }

    fn cry(&self)
    {
        println!("えーんえーん");
    }
}
