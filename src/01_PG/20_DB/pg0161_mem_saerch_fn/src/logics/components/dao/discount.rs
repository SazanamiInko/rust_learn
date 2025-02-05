/////////////////////////////////////
///
/// 割引マスタDAO
/// 
/// //////////////////////////////////
use mysql::{prelude::Queryable, Transaction};
use std::error::Error;

///割引マスタ
#[derive(Clone, Debug)]
pub struct Discount
{
    ///管理番号
    pub id:i32,
    ///割引コード
    pub code:String,
    ///割引名
    pub name:String,
    ///タイムセールフラグ
    pub is_time:bool,
    ///タイムセール開始時
    pub from_hour:u32,
    ///タイムセール終了時
    pub to_hour:u32,
    ///割引率
    pub rate:f32,
    ///削除フラグ
    pub deleteflg:bool
}


impl Discount{


  pub  fn new(id:i32,
              code:String,
              name:String,
              is_time:bool,
              from_hour:u32,
              to_hour:u32,
              rate:f32,
              deleteflg:bool)->Self{

        return  Discount{
            id:id,
            name:name,
            code:code,
            is_time:is_time,
            from_hour:from_hour,
            to_hour:to_hour,
            rate:rate,
            deleteflg:deleteflg
        }       

    }
}

///コンストラクタ
pub fn create(id:i32,
              code:String,
              name:String,
              is_time:bool,
              from_hour:u32,
              to_hour:u32,
              rate:f32,
              deleteflg:bool)->Discount
{

    return Discount
    {
        id:id,
        name:name,
        code:code,
        is_time:is_time,
        from_hour:from_hour,
        to_hour:to_hour,
        rate:rate,
        deleteflg:deleteflg
    };


}

///割引マスタ取得
pub fn get_time_discount(tran:&mut Transaction)->Result<Vec<Discount>,Box<dyn Error>>
{
   
    let  query=r"SELECT id, code,name,is_time,from_hour,to_hour,rate,deleteflg
                      FROM m_discount 
                      WHERE deleteflg=0 
                      AND is_time=1 ";

    let result=tran.query_map(query,
        |(id, code,name,is_time,from_hour,to_hour,rate,deleteflg)|
    {
        Discount
    {
        id:id,
        name:name,
        code:code,
        is_time:is_time,
        from_hour:from_hour,
        to_hour:to_hour,
        rate:rate,
        deleteflg:deleteflg
    }
    })?;

    return Ok(result);                      

}