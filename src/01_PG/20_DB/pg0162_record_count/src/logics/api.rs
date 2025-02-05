//////////////////////////////////////////////
/// 
/// Fericaカード管理
/// 
/// ///////////////////////////////////////////
use crate::logics::common::setting as server_settting; 
use crate::logics::dao::common::connection;
use mysql::prelude::Queryable;
use std::error::Error;

///マスターカードの件数を取得
pub fn get_master_card()->Result<u32,Box<dyn Error>>
{
    //設定ファイルの読み込み
    let server= server_settting::load();
    //サーバーへの接続
    let con= connection::from_config(&server.server);
    
    let mut pool= con.get_connection();

    let query=r"SELECT COUNT(id) FROM m_master_card WHERE deleteflg=0";
    let count=pool.query_first(query)?;

    match count
    {
        None=>
        {
            return Ok(0);
        },
       Some(cun)=>
        {
            return Ok(cun);
        }
    }
}