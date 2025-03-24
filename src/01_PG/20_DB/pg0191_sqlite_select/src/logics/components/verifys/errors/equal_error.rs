/////////////////////////////////
/// 
/// 同値エラー
/// 
/// //////////////////////////////
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
    pub fn from(label:&str,
                label_a:&str,
                label_b:&str)->Self
    {
        let message=format!("{}の同値チェックNG:{}と{}の値が同じです",
                                    label,
                                    label_a,
                                    label_b);

        return EqualError{message:message}; 
    } 
}