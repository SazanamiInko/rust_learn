//////////////////////////////////
/// 
/// 相関検査
/// 
/// //////////////////////////////
use mysql::*;


pub trait LogicalVerify {

    
fn verify(&self,tran:&mut Transaction)->Result<(),Box<dyn std::error::Error>>;

}

