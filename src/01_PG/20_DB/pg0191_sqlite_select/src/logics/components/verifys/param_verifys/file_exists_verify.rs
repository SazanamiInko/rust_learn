////////////////////////////
/// 
/// ファイルの存在チェック
/// 
/// ////////////////////////
use crate::logics::components::{common::util, verifys::errors::file_not_exists_error::FileNotExistsError};
use super::ParamVerify;


//データ定義
pub struct FileExxistsVerify
{
    pub date:String
}

///実装
impl FileExxistsVerify
{
    ///コンストラクタ
    pub fn set(date:&str)->Self
    {
        return FileExxistsVerify
        {
            date:date.to_string(),
        };
    }
}

///検査
impl ParamVerify for FileExxistsVerify 
{
    ///検査する
    fn verify(&self)->Result<(),Box<dyn std::error::Error>> {
        
        if !util::is_exists_sale_data(&self.date)
        {
            let path=util::get_file_name();
            return Err(Box::new(FileNotExistsError::from("売上データ",&path)));
        }
        return Ok(());
    }
}