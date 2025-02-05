///////////////////////////////////
/// 
///カード不存在チェック
/// 
/// ///////////////////////////////
use crate::logics::components::verifys::logical_verifys::LogicalVerify;
use crate::logics::components::verifys::errors::exists_card_error::ExistsCardError;
use crate::logics::components::common::master_card as card_util;
use mysql::*;

///数値検査
pub struct ExistsCardVerify{
    ///試験項目名
    pub label:String,
    ///検査対象
    pub target_card:String,
   
} 

///実装
impl  ExistsCardVerify{


    ///コンストラクタ
    pub fn set(label:&str,
        target_card:&str)->Self
    {
        return ExistsCardVerify{
            label:label.to_string(),
            target_card:target_card.to_string()
        };
    }
}

///検査
impl LogicalVerify for ExistsCardVerify
{
    ///検査する
    fn verify(&self,tran:&mut Transaction)->Result<(),Box<dyn std::error::Error>> {
        
      
        if card_util::has_master(self.target_card.as_str(), tran){
            return Err(Box::new(ExistsCardError::from(self)));
        }

        Ok(())
    }
}