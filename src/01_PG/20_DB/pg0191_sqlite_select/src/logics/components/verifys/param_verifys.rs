////////////////////////////////////
/// 
/// 検査トレイト
/// 
/// /////////////////////////////////
use std::error::Error;

//モジュール一覧
pub mod equal_verify;
pub mod range_verify;
pub mod file_exists_verify;


///検査トレイト
pub trait ParamVerify
{
    fn verify(&self)->Result<(),Box<dyn Error>>;
}

