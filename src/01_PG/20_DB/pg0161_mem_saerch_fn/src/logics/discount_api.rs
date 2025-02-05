/////////////////////////////////////
/// 
/// 割引API
/// 
/// /////////////////////////////////
use mysql::TxOpts;
use crate::logics::components::dao::discount::Discount;
use crate::logics::components::common::setting;
use crate::logics::components::dao::common::connection;
use super::components::{find_discount_component::FindDiscountComponent, get_discount_component::GetDiscountComponent, Component, GetComponent};
use std::error::Error;

///割引情報初期化
pub fn init_discount()->Result<(),Box<dyn Error>>
{
   let component=GetDiscountComponent::new();
   let config =setting::load();
   let con=connection::from_config(&config.server);
   let mut p=con.get_connection();
   let mut tran=p.start_transaction(TxOpts::default())?;
   
   component.execute(&mut tran)?;
   return Ok(());
}

///割引情報取得
pub fn get_discount(hour:u32)->Result<Option<Discount>,Box<dyn Error>>
{
   let component=FindDiscountComponent::new(hour);
   let config =setting::load();
   let con=connection::from_config(&config.server);
   let mut p=con.get_connection();
   let mut tran=p.start_transaction(TxOpts::default())?;
   component.check_param()?;
   let data=component.execute(&mut tran)?;

   if data.len()==0
   {
      return Ok(None);
   }

   return Ok(Some(data[0].clone()));
}