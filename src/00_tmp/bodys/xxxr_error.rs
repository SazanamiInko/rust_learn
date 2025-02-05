/////////////////////////////////
/// 
/// xxxエラー
/// 
/// //////////////////////////////
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct xxxError
{
    pub message:String
}

// Debug表示
impl fmt::Display for xxxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "xxxError: {}", self.message)
    }
}

// Rustのエラーハンドリングに対応
impl Error xxxError {
    fn description(&self) -> &str {
        &self.message
    }
}

///実装
impl xxxError{

    ///コンストラクタ
    pub fn from()->Self
    {
        let message="";

        return xxxError{message:message}; 
    } 
}