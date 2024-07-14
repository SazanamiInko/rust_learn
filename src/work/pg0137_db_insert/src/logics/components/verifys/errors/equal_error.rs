/////////////////////////////////
/// 
/// 同値エラー
/// 
/// //////////////////////////////
use crate::logics::components::verifys::param_verifys::equal_verify::EqualVerify;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct EqualError
{
    pub message:String
}

// Displayトレイトを実装することで
impl fmt::Display for EqualError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EqualError: {}", self.message)
    }
}

// Errorトレイトを実装することで、Rustのエラーハンドリングに対応
impl Error for EqualError {
    fn description(&self) -> &str {
        &self.message
    }
}

///実装
impl EqualError{

    ///コンストラクタ
    pub fn from(equal_verify:&EqualVerify)->Self
    {
        let message=format!("{}項目/同値チェックNG:{}～{}の間の値にしてください。"
                                    ,equal_verify.label
                                    ,equal_verify.label_a
                                    ,equal_verify.label_b);

        return EqualError{message:message}; 
    } 
}