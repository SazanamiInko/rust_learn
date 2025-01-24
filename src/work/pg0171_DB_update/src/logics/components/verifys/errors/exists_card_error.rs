//////////////////////////////////////////////
/// 
/// カード存在エラー
/// 
/// //////////////////////////////////////////
use crate::logics::components::verifys::logical_verifys::exists_card_verify::ExistsCardVerify;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ExistsCardError
{
    pub message:String
}

// Displayトレイトを実装することで
impl fmt::Display for ExistsCardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ExistsCardError: {}", self.message)
    }
}

// Errorトレイトを実装することで、Rustのエラーハンドリングに対応
impl Error for ExistsCardError {
    fn description(&self) -> &str {
        &self.message
    }
}

///実装
impl ExistsCardError{

    ///コンストラクタ
    pub fn from(exists_card_verify:&ExistsCardVerify)->Self
    {
        let message=format!("カード存在チェックNG:{}は登録済みです"
                                    ,exists_card_verify.label);

        return ExistsCardError{message:message}; 
    } 
}