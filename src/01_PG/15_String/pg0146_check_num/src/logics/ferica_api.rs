/////////////////////////////////////////
/// 
/// Ferica API
/// 
/// //////////////////////////////////////
use std::error::Error;
use crate::logics::verifys::num_check_verify;
use super::traits::verify::Verify;

///検査する
pub fn check(mid:&str)->Result<(),Box<dyn Error>>
{
    num_check_verify::NumVerify::set(mid, "カード検査")
                                .verify()
                                ?;

    return Ok(());
}