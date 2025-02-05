////////////////////////////////////
/// 
/// 検査トレイト
/// 
/// /////////////////////////////////
use std::error::Error;

//モジュール一覧

///検査トレイト
pub trait Verify
{
    fn verify(&self)->Result<(),Box<dyn Error>>;
}

