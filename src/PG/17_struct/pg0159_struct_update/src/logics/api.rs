///////////////////////////////
/// 
/// API
/// 
/// ////////////////////////////
use crate::logics::benefit;

///株主優待決定
pub fn decide_benegit(work_banefit:&mut benefit::Benefit)
{
    work_banefit.benefit=1000;
}