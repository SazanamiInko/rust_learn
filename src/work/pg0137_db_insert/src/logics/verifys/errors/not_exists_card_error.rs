//////////////////////////////////////////////
/// 
/// カード存在エラー
/// 
/// //////////////////////////////////////////
use crate::logics::verifys::not_exists_card_verify::NotExistsCardVerify;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct NotExistsCardError
{
    pub message:String
}

// Displayトレイトを実装することで
impl fmt::Display for NotExistsCardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NotExistsCardError: {}", self.message)
    }
}

// Errorトレイトを実装することで、Rustのエラーハンドリングに対応
impl Error for NotExistsCardError {
    fn description(&self) -> &str {
        &self.message
    }
}

///実装
impl NotExistsCardError{

    ///コンストラクタ
    pub fn from(not_exists_card_verify:&NotExistsCardVerify)->Self
    {
        let message=format!("{}項目/カード存在チェックNG:{}は登録済みです"
                                    ,not_exists_card_verify.label
                                    ,not_exists_card_verify.target_card);

        return NotExistsCardError{message:message}; 
    } 
}