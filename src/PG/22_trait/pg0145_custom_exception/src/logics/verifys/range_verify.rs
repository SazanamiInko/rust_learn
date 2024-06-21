////////////////////////
/// 
/// 範囲チェック
/// 
/// /////////////////////

use crate::logics::errors;
use crate::logics::traits;
use std::error::Error;

///範囲構造体
pub struct RangeVerify{
    ///最小
    pub mini:i32,
    ///最大
    pub max:i32,
    ///検査値
    pub target:i32,
    ///試験項目
    pub label:String
}

///検査トレイトのエラー
impl traits::verify::Verify for RangeVerify{

  ///検査
  fn verify(&self)->Result<(),Box<dyn Error>>
  {

    let mut judge=false;
    if self.target<self.mini
    {
      judge=true;
    }
    else if self.target>self.max
    {
      judge=true;
    }

    if judge
    {
     return Err(Box::new(errors::range_error::from(self)));
    }

    return Ok(());
  }
}

///実装
impl RangeVerify{

  ///試験項目のセット
  pub fn set(&mut self,label:String,target:i32)
  {
    self.label=label.clone();
    self.target=target;
  }

}

///コンストラクタ
pub fn create(mini:i32,max:i32)->RangeVerify
{
  return RangeVerify{
    mini:mini,
    max:max,
    target:0,
    label:String::from("")
  }
}