//////////////////////////////////////////
/// 
/// 割引マスタ検索コンポネント
/// 
/// //////////////////////////////////////
use crate::logics::components::dao::discount::Discount;
use crate::logics::components::GetComponent;
use crate::logics::components::resources::discounts;
use crate::logics::components::verifys::param_verifys::max_verify::MaxVerify;
use std::error::Error;
use mysql::Transaction;

use super::verifys::param_verifys::ParamVerify;



///割引マスタ検索コンポネント
pub struct FindDiscountComponent
{
    ///時間
    pub hour:u32
} 


impl FindDiscountComponent
{
    pub fn new(hour:u32)->Self
    {
        return FindDiscountComponent
        {
            hour:hour
        } 
    }
}

///コンポネントの実装
impl GetComponent<Discount> for FindDiscountComponent
{
     ///値チェック
     fn check_param(&self)->Result<(),Box<dyn Error>>
     {
       MaxVerify::set("割引時刻",
                       24, 
                       self.hour)
        .verify()
        ?;

          return Ok(());
     }
 
     ///相関チェック
     fn check_logical(&self,tran:&mut Transaction)->Result<(),Box<dyn Error>>
     {
         return Ok(());
     }
 
     ///実行
     fn execute(&self,tran:&mut Transaction)->Result<Vec<Discount>,Box<dyn Error>>
     {
        //検索結果のクリア
        let mut result=Vec::<Discount>::new();   
        //リソースの検索
        let discounts=discounts::DISCOUNTS.lock().unwrap();
        let resources= discounts.iter();
        
        for discount in resources
        {
            if discount.from_hour<=self.hour
            && discount.to_hour>=self.hour
            {
                    result.push(discount.clone());
            }
        }

        return Ok(result);
        
     }
}
