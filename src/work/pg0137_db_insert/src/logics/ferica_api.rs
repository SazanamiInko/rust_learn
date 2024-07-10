/////////////////////////////////
/// 
/// メンテナンスFerica管理
/// 
/// /////////////////////////////
use crate::logics::verifys::equal_verify::EqualVerify;
use crate::logics::verifys::length_verify::LengthVerify;
use crate::logics::verifys::not_number_verify::NotNumberVerify;
use std::error::Error;
use super::traits::verify::Verify;

///管理Fericaの登録
pub fn add_ferica(new_ferica:&str,add_ferica:&str)->Result<u32,Box<dyn Error>>
{
    //トランザクションの作成
    //登録しようとするFericaが登録済みかをチェック
    //登録者Ferica登録済みチェック
    //登録者Ferica登録者の存在チェック
    //登録者Ferica承認者チェック
    //Fericaの登録
    return Ok(1);
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