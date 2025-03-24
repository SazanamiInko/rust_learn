//////////////////////
/// 
/// 売上データモデル
/// 
/// //////////////////
use crate::logics::components::common::const_list;

///データ定義
pub struct SaleDataModel
{
    //発駅
    pub take_on:i32,
    //着駅
    pub take_off:i32,
    //値段
    pub price:i32,
    //方向
    pub distance:i32
}

//実装
impl SaleDataModel
{
    //コンストラクタ
    pub fn new(take_on:i32,take_off:i32)->Self
    {
       let mut area=take_off-take_on;
       let mut d=const_list::SOUTH_DISTANCE;
       if take_on>take_off
       {
            area=-1*area;
            d=const_list::NORTH_DISTANCE;
       }

       return SaleDataModel
       {
            take_on:take_on,
            take_off:take_off,
            price:const_list::CHARGE*area,
            distance:d
       };
    }
}