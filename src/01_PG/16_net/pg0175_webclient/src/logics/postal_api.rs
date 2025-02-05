/////////////////////////////
/// 
/// 郵便番号API
/// 
/// //////////////////////////
use tokio;
use reqwest;
use std::error::Error;

use super::{components::verifys::status_verify::StatusVerify, traits::param_verify_trait::Verify};

///郵便番号から住所を引く
#[tokio::main]
pub async fn get_address(zip_no:&str)->Result<String,Box<dyn Error>>
{
  //URL
  let urltemp=String::from(r"https://zipcloud.ibsnet.co.jp/api/search?zipcode=@zipcode");
  let url=urltemp.replace("@zipcode", zip_no);
  let response=reqwest::get(url)
                         .await
                         .unwrap(); 
  let verify_component=StatusVerify::from(response.status().clone());
  _=verify_component.verify()?;
  let response_body= response.text()
                                     .await
                                      ?;
  return Ok(response_body);

}