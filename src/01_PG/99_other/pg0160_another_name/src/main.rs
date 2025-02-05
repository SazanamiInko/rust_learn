/////////////////////////////////////////
/// 
/// 別名
/// 
/// /////////////////////////////////////
mod  logics;
use crate::logics::common::stock as stock_common;
use crate::logics::structs::stock as stock_structs;

///メイン関数
fn main() {
    let stocker=stock_structs::Stocker::new("112233445566",2012,107);
    stock_common::decide_benefit();
}
