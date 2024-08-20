///////////////////////////////////
/// 
/// 最大値検査
/// 
/// ///////////////////////////////
use crate::logics::components::verifys::errors::max_error::MaxError;
use crate::logics::components::verifys::param_verifys::ParamVerify;

///数値検査
pub struct MaxVerify{
    ///試験項目名
    pub label:String,
    ///比較する値A
    pub max:u32,
    pub target:u32
   
} 

///実装
impl MaxVerify{
    pub fn set(label:&str,
               max:u32,
               target:u32)->Self
    {
        return MaxVerify{
                        label:label.to_string(),
                        max:max,
                        target:target
                        };
    }
}

///検査
impl ParamVerify for MaxVerify
{
    fn verify(&self)->Result<(),Box<dyn std::error::Error>> {
        
        if self.target>self.max
        {
            return Err(Box::new(MaxError::from(self)));
        }
        return Ok(());
    }
}