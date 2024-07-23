//////////////////////////////////////////////
/// 
/// 承認者カードエラー
/// 
/// //////////////////////////////////////////
use crate::logics::components::verifys::logical_verifys::auth_verify::AuthVerify;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct AuthError
{
    pub message:String
}

// Displayトレイトを実装することで
impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AuthError: {}", self.message)
    }
}

// Errorトレイトを実装することで、Rustのエラーハンドリングに対応
impl Error for AuthError {
    fn description(&self) -> &str {
        &self.message
    }
}

///実装
impl AuthError{

    ///コンストラクタ
    pub fn from(auth_verify:&AuthVerify)->Self
    {
        let message=format!("承認カードチェックNG:{}は登録できるカードではあいません"
                                    ,auth_verify.target_card);

        return AuthError{message:message}; 
    } 
}