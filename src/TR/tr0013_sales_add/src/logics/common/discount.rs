//////////////////////////////////////////////
/// 
/// 割引
/// 
/// ///////////////////////////////////////////
use crate::logics::dao::*;
use log::info;

///割引を設定
pub fn set_discount(sale:&mut sale::Sale,discount:&discount::Discount)
{
    info!("set_discount start");
    sale.discount=discount.code.clone();
    sale.rate=discount.rate;
    info!("set_discount end");
}

///時間に適用した割引を取得する
pub fn get_timesale(hour:&u32,discounts:&Vec<discount::Discount>)->Option<discount::Discount>
{
    info!("get_timesale start");
    let target:Vec<discount::Discount>= discounts.iter()
    .cloned()
    .filter(|record| *hour>=record.from_hour&&*hour<record.to_hour)
    .collect();
    
    if target.len()>0 {
        info!("get_timesale has end");
        return Some(target[0].clone());
    }

    info!("get_timesale no end");
    return None;
}