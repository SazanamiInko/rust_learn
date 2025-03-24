/////////////////////////////////
/// 
/// ファイル不在エラー
/// 
/// //////////////////////////////
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct FileNotExistsError
{
    pub message:String
}

// Displayトレイトを実装することで
impl fmt::Display for FileNotExistsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FileNotExistsError: {}", self.message)
    }
}

// Errorトレイトを実装することで、Rustのエラーハンドリングに対応
impl Error for FileNotExistsError {
    fn description(&self) -> &str {
        &self.message
    }
}

///実装
impl FileNotExistsError{

    ///コンストラクタ
    pub fn from(filename:&str,path:&str)->Self
    {
        let message=format!("{}の存在チェックNG:ファイルが存在しません。{}",
                                    filename,
                                    path);

        return FileNotExistsError{message:message}; 
    } 
}