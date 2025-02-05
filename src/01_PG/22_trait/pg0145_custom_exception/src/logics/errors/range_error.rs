//////////////////////////////////////
/// 
/// 範囲外例外
/// 
/// ///////////////////////////////////

use crate::logics::verifys::range_verify;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct RangeError
{
    pub message:String
}

// Displayトレイトを実装することで、エラーのフォーマット方法を定義
impl fmt::Display for RangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RangeError: {}", self.message)
    }
}

// Errorトレイトを実装することで、Rustのエラーハンドリングに対応
impl Error for RangeError {
    fn description(&self) -> &str {
        &self.message
    }
}


///コンストラクタ
pub fn from(range_verify:&range_verify::RangeVerify)->RangeError
{
    let message=format!("{}項目/範囲チェックNG:{}～{}の間の値にしてください。"
                                ,range_verify.label
                                ,range_verify.mini
                                ,range_verify.max);

    return RangeError{message:message}; 
} 