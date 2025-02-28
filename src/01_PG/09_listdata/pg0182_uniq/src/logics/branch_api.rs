/////////////////////////
/// 
/// 支店API
/// 
/// //////////////////////
use crate::logics::components::common::branch;
use std::error::Error;

///支店リスト取得
pub fn get_branch_list()->Result<Vec<String>,Box<dyn Error>>
{
    return branch::get_branch_list();
}
