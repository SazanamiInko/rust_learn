//////////////////////////////
///
/// 割引マスタ取得コンポーネント
/// 
/// //////////////////////////
use crate::logics::components::Component;
use crate::logics::components::dao::discount;
use std::error::Error;
use mysql::Transaction;
use crate::logics::components::resources;

///割引情報取得コンポネント
pub struct GetDiscountComponent
{
}

///実装
impl GetDiscountComponent
{
    ///コンストラクタ
    pub fn new()->Self
    {
        return GetDiscountComponent{};
    }
}

impl Component for GetDiscountComponent
{
    ///値チェック
    fn check_param(&self)->Result<(),Box<dyn Error>>
    {
         return Ok(());
    }

    ///相関チェック
    fn check_logical(&self,tran:&mut Transaction)->Result<(),Box<dyn Error>>
    {
        return Ok(());
    }

    ///実行
    fn execute(&self,tran:&mut Transaction)->Result<(),Box<dyn Error>>
    {
        //割引情報をDBから取得
        let discount_db=discount::get_time_discount(tran)?;
        let mut discounts=resources::discounts::DISCOUNTS.lock().unwrap();
        *discounts=discount_db;

        return Ok(());
    }

}