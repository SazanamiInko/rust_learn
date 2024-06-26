//////////////////////////////////////
/// 
/// 数字以外例外
/// 
/// ///////////////////////////////////

use crate::logics::verifys::num_check_verify;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct NotNumError
{
    pub message:String
}

// Displayトレイトを実装することで、エラーのフォーマット方法を定義
impl fmt::Display for NotNumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RangeError: {}", self.message)
    }
}

// Errorトレイトを実装することで、Rustのエラーハンドリングに対応
impl Error for NotNumError {
    fn description(&self) -> &str {
        &self.message
    }
}


///コンストラクタ
pub fn from(num_verify:&num_check_verify::NumVerify)->NotNumError
{
    let message=format!("{}項目/数字チェックNG:{}に数字以外の文字があります。"
                                ,num_verify.label
                                ,num_verify.target);

    return NotNumError{message:message}; 
} 