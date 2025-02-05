///////////////////////////////////////////
/// 
///   リソース（割引情報）
/// 
/// ///////////////////////////////////////
use crate::logics::components::dao::discount;
use once_cell::sync::Lazy;
use std::sync::Mutex;


///静的変数
pub static DISCOUNTS: Lazy<Mutex<Vec<discount::Discount>
>> = Lazy::new(|| {
    //空のベクターを最初に登録
    Mutex::new(Vec::new())
});