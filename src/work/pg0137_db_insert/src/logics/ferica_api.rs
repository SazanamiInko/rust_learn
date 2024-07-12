/////////////////////////////////
/// 
/// メンテナンスFerica管理
/// 
/// /////////////////////////////
use crate::logics::verifys::equal_verify::EqualVerify;
use crate::logics::verifys::length_verify::LengthVerify;
use crate::logics::verifys::not_number_verify::NotNumberVerify;
use crate::logics::verifys::not_exists_card_verify::NotExistsCardVerify;
use crate::logics::verifys::auth_verify::AuthVerify;
use std::error::Error;
use super::traits::logical_verify::LogicalVerify;
use super::traits::verify::Verify;

///管理Fericaの登録
pub fn add_ferica(new_ferica:&str,add_ferica:&str)->Result<u32,Box<dyn Error>>
{
    //設定ファイルの読み込み
   
    //トランザクションの作成
    //登録しようとするFericaが登録済みかをチェック
    //登録者Ferica登録済みチェック
    //登録者Ferica登録者の存在チェック
    //登録者Ferica承認者チェック
    //Fericaの登録
    return Ok(1);
}

///相関チェック
pub fn check_logical(add_card:&str,adder_card:&str,tran:&mut Transaction)->Result<(),Box<dyn Error>>
{
  //カード存在チェック
  NotExistsCardVerify::set("カード登録", add_card)
  .verify(tran)
  ?;

  //カード承認者
  AuthVerify::set("カード登録者", adder_card)
  .verify(tran)
  ?;

}

///Fericaのチェック
pub fn check(add_card:&str,adder_card:&str)->Result<(),Box<dyn Error>>
{
    const LEN:u32=16;

    //同値チェック
    EqualVerify::set("登録するカードと登録者のチェック",
                     "登録するカード", 
                     add_card, 
                     "登録者のカード", 
                     adder_card)
                     .verify()
                    ?;
    
    //文字列長チェック
    LengthVerify::set("登録するカード",
                      add_card,
                      LEN)
                    .verify()
                    ?;     

    LengthVerify::set("登録者のカード",
                      adder_card,
                      LEN)
                    .verify()
                    ?;                 
    //数字チェック        
    
    NotNumberVerify::set("登録するカード",
    add_card) 
    .verify()
    ?;   
    
    NotNumberVerify::set("登録者のカード",
    adder_card) .verify()
    ?;   


   return Ok(());
}