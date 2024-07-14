///////////////////////////////////
/// 
/// 同値検査
/// 
/// ///////////////////////////////
use crate::logics::components::verifys::errors::equal_error::EqualError;
use crate::logics::components::verifys::param_verifys::ParamVerify;

///同値検査
pub struct EqualVerify{
    ///試験項目名
    pub label:String,
    ///比較する値A
    pub value_a:String,
    ///Aの名称
    pub label_a:String,
    ///比較する値B
    pub value_b:String,
    ///Bの名称
    pub label_b:String
} 

///実装
impl EqualVerify{

    ///コンストラクタ
    pub fn set(label:&str,
               label_a:&str,
               value_a:&str,
               label_b:&str,
               value_b:&str)->Self
    {
        return EqualVerify{
            label:label.to_string(),
            label_a:label_a.to_string(),
            value_a:value_a.to_string(),
            label_b:label_b.to_string(),
            value_b:value_b.to_string()
        };
    }
}

///検査
impl ParamVerify for EqualVerify
{
    ///検査する
    fn verify(&self)->Result<(),Box<dyn std::error::Error>> {
        
        if self.value_a==self.value_b{
            return Err(Box::new(EqualError::from(self)));
        }

        Ok(())
    }
}