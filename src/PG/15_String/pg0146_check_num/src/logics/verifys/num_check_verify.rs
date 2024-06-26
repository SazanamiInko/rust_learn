////////////////////////
/// 
/// 数値チェック
/// 
/// /////////////////////

use crate::logics::verifys::errors;
use crate::logics::traits::verify;
use std::error::Error;

///数値検査
pub struct NumVerify{
 ///検査値
 pub target:String,
 ///試験項目
 pub label:String   
}

///継承
impl verify::Verify for NumVerify
{
    ///検査する
    fn verify(&self)->Result<(),Box<dyn Error>>
    {
        let check=self.target.chars()
                            .all(|data|data.is_ascii_digit());

        if !check
        {
            return Err(Box::new(errors::num_error::from(self)));
        }
        
        return Ok(());
    }
}

impl NumVerify
{
    ///コンストラクタ
    pub fn set(target:&str,label:&str)->Self
    {
        return NumVerify{
            target:target.to_string(),
            label:label.to_string()
        }

    }
}