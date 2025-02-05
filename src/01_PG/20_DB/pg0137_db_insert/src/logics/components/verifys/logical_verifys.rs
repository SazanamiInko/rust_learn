//////////////////////////////////
/// 
/// 相関検査
/// 
/// //////////////////////////////
use mysql::*;

//モジュール一覧
pub mod auth_verify;
pub mod not_exists_card_verify;

pub trait LogicalVerify {

    
fn verify(&self,tran:&mut Transaction)->Result<(),Box<dyn std::error::Error>>;

}

