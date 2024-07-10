///////////////////////////////////
/// 
///数値検査
/// 
/// ///////////////////////////////
use crate::logics::traits::verify::Verify;
use crate::logics::verifys::errors::not_number_error::NotNumberError;
///数値検査
pub struct NotNumberVerify{
    ///試験項目名
    pub label:String,
    ///比較する値A
    pub target:String,
   
} 

///実装
impl NotNumberVerify{

    ///コンストラクタ
    pub fn set(label:&str,
               target:&str)->Self
    {
        return NotNumberVerify{
            label:label.to_string(),
            target:target.to_string()
        };
    }
}

///検査
impl Verify for NotNumberVerify
{
    ///検査する
    fn verify(&self)->Result<(),Box<dyn std::error::Error>> {
        
       let val:String=self.target.chars()
        .filter(|char_data|{*char_data<'0'||*char_data>'9'})
        .collect();

        if val.len()>0{
            return Err(Box::new(NotNumberError::from(self)));
        }

        Ok(())
    }
}