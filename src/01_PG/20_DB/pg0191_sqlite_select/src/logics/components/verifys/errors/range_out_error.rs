/////////////////////////////////
/// 
/// 範囲外エラー
/// 
/// //////////////////////////////
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct RangeOutError
{
    pub message:String
}

// メッセージの表示
impl fmt::Display for RangeOutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RangeOutError: {}", self.message)
    }
}

// Error機能
impl Error for RangeOutError {
    fn description(&self) -> &str {
        &self.message
    }
}

///実装
impl RangeOutError{

    ///コンストラクタ
    pub fn from(label:&str,mini:i32,max:i32)->Self
    {
        let message=format!("{}範囲チェックNG:{}から{}の値を指定してください。"
                                    ,label
                                    ,mini
                                    ,max);

        return RangeOutError{message:message}; 
    } 
}