/////////////////////////////////
/// 
/// 数値以外エラー
/// 
/// //////////////////////////////
use crate::logics::verifys::not_number_verify::NotNumberVerify;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct NotNumberError
{
    pub message:String
}

// Displayトレイトを実装することで
impl fmt::Display for NotNumberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NotNumberError: {}", self.message)
    }
}

// Errorトレイトを実装することで、Rustのエラーハンドリングに対応
impl Error for NotNumberError {
    fn description(&self) -> &str {
        &self.message
    }
}

///実装
impl NotNumberError{

    ///コンストラクタ
    pub fn from(not_number_verify:&NotNumberVerify)->Self
    {
        let message=format!("{}項目/数値エラーチェックNG:数値のデータにしてください。"
                                    ,not_number_verify.label);

        return NotNumberError{message:message}; 
    } 
}