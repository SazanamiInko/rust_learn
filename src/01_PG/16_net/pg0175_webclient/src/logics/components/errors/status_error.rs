///////////////////////////////
/// 
/// 状態エラー
/// 
/// ///////////////////////////
use std::error::Error;
use std::fmt;
use reqwest::StatusCode;

#[derive(Debug)]
pub struct StatusError
{
    pub message:String
}

impl fmt::Display for StatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StatusError: {}", self.message)
    }
}

// Errorトレイトを実装することで、Rustのエラーハンドリングに対応
impl Error for StatusError {
    fn description(&self) -> &str {
        &self.message
    }
}
///実装
impl StatusError{

    ///コンストラクタ
    pub fn from(status_code:StatusCode)->Self
    {
        let message=format!("Staatusが{}が返ってきました",status_code);

        return StatusError{message:message}; 
    } 
}