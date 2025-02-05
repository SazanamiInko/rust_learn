//////////////////////////////////
/// 
/// ねこ構造体
/// 
/// ///////////////////////////////

use crate::logics::animal::Animal;

///ねこ構造体
pub struct Cat{}

///実装
impl Animal for Cat
{
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

///コンストラクタ
pub fn create()->Cat
{
    return Cat{};
}