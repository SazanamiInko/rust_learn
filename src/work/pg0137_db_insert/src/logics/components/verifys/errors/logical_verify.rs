///////////////////////////////////////
/// 
/// 相関検査トレイト
/// 
/// ///////////////////////////////////
use mysql::*;
use std::error::Error;

///相関検査トレイト

pub trait LogicalVerify
{
    fn verify(&self,tran:&mut Transaction)->Result<(),Box<dyn Error>>;
}