/////////////////////////////////////////
/// 
/// 最大以上エラー
/// 
/// /////////////////////////////////////

use crate::logics::components::verifys::param_verifys::max_verify::MaxVerify;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct MaxError
{
    pub message:String
}

// Displayトレイトを実装することで
impl fmt::Display for MaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EqualError: {}", self.message)
    }
}

// Errorトレイトを実装することで、Rustのエラーハンドリングに対応
impl Error for MaxError {
    fn description(&self) -> &str {
        &self.message
    }
}

//実装
impl MaxError{

    ///コンストラクタ
    pub fn from(max_verify:&MaxVerify)->Self
    {
        let message=format!("最大値チェックNG:{}は{}以下の値を指定してください"
                                    ,max_verify.label
                                    ,max_verify.max);

        return MaxError{message:message}; 
    } 
}