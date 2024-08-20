////////////////////////////////////
/// 
/// 検査トレイト
/// 
/// /////////////////////////////////
use std::error::Error;

//モジュール一覧
pub mod equal_verify;
pub mod length_verify;
pub mod not_number_verify;
pub mod max_verify;

///検査トレイト
pub trait ParamVerify
{
    fn verify(&self)->Result<(),Box<dyn Error>>;
}

