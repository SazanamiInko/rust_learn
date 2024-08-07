/////////////////////////////////
/// 
/// 文字列長エラー
/// 
/// //////////////////////////////

use crate::logics::components::verifys::param_verifys::length_verify::LengthVerify;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct LengthError
{
    pub message:String
}

// Displayトレイトを実装することで
impl fmt::Display for LengthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LengthError: {}", self.message)
    }
}

// Errorトレイトを実装することで、Rustのエラーハンドリングに対応
impl Error for LengthError {
    fn description(&self) -> &str {
        &self.message
    }
}

///実装
impl LengthError{

    ///コンストラクタ
    pub fn from(length_verify:&LengthVerify)->Self
    {
        let message=format!("文字列長チェックNG:{}は{}文字の引数を指定してください"
                                    ,length_verify.label
                                    ,length_verify.length);

        return LengthError{message:message}; 
    } 
}