//////////////////////////////
/// 
/// 株主優待
/// 
/// //////////////////////////
use crate::logics::stocker;

///株主優待
pub struct Benefit
{
     ///株主番号
     id:String,
     ///取得年
     enter_year:u32,
     ///株数
     stocks:u32,
     ///優待
     benefit:u32
}

///実装
impl Benefit{

    ///コンストラクタ
    pub fn new (stocker:&stocker::Stocker)->Self
    {
        return Benefit
        {
            id:stocker.id,
            enter_year:stocker.enter_year,
            stocks:stocker.stocks,
            benefit:0
        };
    }

}