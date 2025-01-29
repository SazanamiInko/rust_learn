///////////////////////////////////
/// 
/// 文字列長検査
/// 
/// ///////////////////////////////
use crate::logics::components::verifys::errors::length_error::LengthError;
use crate::logics::components::verifys::param_verifys::ParamVerify;

///同値検査
pub struct LengthVerify{
    ///試験項目名
    pub label:String,
    ///比較する値A
    pub target:String,
    ///Aの名称
    pub length:u32,
  
} 

///実装
impl LengthVerify{

    ///コンストラクタ
    pub fn set(label:&str,
               target:&str,
               length:u32)->Self
    {
        return LengthVerify{
            label:label.to_string(),
            target:target.to_string(),
            length:length
        };
    }
}

///検査
impl ParamVerify for LengthVerify
{
    ///検査する
     fn verify(&self)->Result<(),Box<dyn std::error::Error>> {
        
        if self.target.len() as u32!=self.length{
            return Err(Box::new(LengthError::from(self)));
        }

        Ok(())
    }
}