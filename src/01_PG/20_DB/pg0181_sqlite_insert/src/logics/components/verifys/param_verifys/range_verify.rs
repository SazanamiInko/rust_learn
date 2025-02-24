///////////////////////////////////
/// 
/// 範囲検査
/// 
/// ///////////////////////////////
use crate::logics::components::verifys::errors::range_out_error::RangeOutError;
use crate::logics::components::verifys::param_verifys::ParamVerify;

///同値検査
pub struct RangeVerify{
    ///試験項目名
    pub label:String,
    ///比較する値A
    pub target:i32,
    ///最小値
    pub mini:i32,
    ///最大値
    pub max:i32

} 

///実装
impl RangeVerify{

    ///コンストラクタ
    pub fn set(label:&str,
               target:i32,
               mini:i32,
               max:i32)->Self
    {
        return RangeVerify{
            label:label.to_string(),
            target:target,
            max:max,
            mini:mini
        };
    }
}

///検査
impl ParamVerify for RangeVerify
{
    ///検査する
     fn verify(&self)->Result<(),Box<dyn std::error::Error>> {
        
       if self.target<self.mini
       {
            return Err(Box::new(RangeOutError::from(self.label.as_str(),self.mini,self.max)));
       }

       if self.target>self.max
       {
            return Err(Box::new(RangeOutError::from(self.label.as_str(),self.mini,self.max)));
       }

        Ok(())
    }
}