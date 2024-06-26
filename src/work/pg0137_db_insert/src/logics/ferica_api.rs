/////////////////////////////////
/// 
/// メンテナンスFerica管理
/// 
/// /////////////////////////////

use std::error::Error;

///管理Fericaの登録
pub fn add_ferica(new_ferica:&str,add_ferica:&str)->Result<u32,Box<dyn Error>
{
    //トランザクションの作成
    //登録しようとするFericaが登録済みかをチェック
    //登録者Ferica登録済みチェック
    //登録者Ferica登録者の存在チェック
    //登録者Ferica承認者チェック
    //Fericaの登録
}

///Fericaのチェック
pub fn check(mid:&str)->Result<(),Box<dyn Error>
{
    
}