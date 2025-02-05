//////////////////////////
/// 
/// 通信状態検査
/// 
/// ///////////////////////
use crate::logics::traits::param_verify_trait::Verify;
use crate::logics::components::errors::status_error::StatusError;
use reqwest:: StatusCode;

///状態検査
pub struct StatusVerify
{
    status_code:StatusCode
}

//実装
impl StatusVerify
{
    ///コンストラクタ
    pub fn from(status_code:StatusCode)->Self
    {
        return StatusVerify{
            status_code:status_code,
        }
    }
}

//検査トレイト
impl Verify for StatusVerify {

    fn verify(&self)->Result<(),Box<dyn std::error::Error>> {
        
        if self.status_code.is_success()
        {
            return Ok(());
        }

        return Err(Box::new(StatusError::from(self.status_code)));
    }
    
}