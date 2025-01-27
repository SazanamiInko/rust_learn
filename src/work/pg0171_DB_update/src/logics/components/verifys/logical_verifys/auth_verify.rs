///////////////////////////////////
/// 
/// 承認者チェック
/// 
/// ///////////////////////////////
use crate::logics::components::verifys::logical_verifys::LogicalVerify;
use crate::logics::components::verifys::errors::auth_error::AuthError;
use crate::logics::components::common::master_card as card_util;
use mysql::*;

///数値検査
pub struct AuthVerify{
    ///試験項目名
    pub label:String,
    ///動作
    pub verbe:String,
    ///検査対象
    pub target_card:String,
   
} 

///実装
impl AuthVerify{

    ///コンストラクタ
    pub fn set(label:&str,
        verbe:&str,
        target_card:&str)->Self
    {
        return AuthVerify{
            label:label.to_string(),
            verbe:verbe.to_string(),
            target_card:target_card.to_string()
        };
    }
}

///検査
impl LogicalVerify for AuthVerify
{
    ///検査する
    fn verify(&self,tran:&mut Transaction)->Result<(),Box<dyn std::error::Error>> {
           
        if !card_util::is_auth(self.target_card.as_str(), tran){
            return Err(Box::new(AuthError::from(self)));
        }

        Ok(())
    }
}