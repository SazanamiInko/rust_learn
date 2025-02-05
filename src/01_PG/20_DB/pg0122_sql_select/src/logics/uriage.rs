////////////////////////////////////////
/// 
/// 売り上げ管理
/// 
/// /////////////////////////////////////
mod connection;
mod goods;

use mysql::prelude::*;


///売り上げDBの商品マスタを取得
pub fn get_goods()->Vec<goods::Goods>
{
    let conn=connection::create();
    let mut conn = conn.get_connection();
    let query="SELECT id, name FROM m_goods";

    return conn.query_map(query,|(id,name)|
                            {
                                goods::create(id,name)
                            }
                        ).expect("クエリの実行に失敗しました");

}