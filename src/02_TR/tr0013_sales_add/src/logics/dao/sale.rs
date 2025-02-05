////////////////////////////////////////////
/// 
/// 売り上げD
/// 
/// /////////////////////////////////////////
use std::error::Error;
use log::info;
use mysql::*;
use mysql::prelude::*;
use crate::logics::common::locker;

///売り上げ
pub struct Sale{
    ///管理番号
    pub id :i32,
    ///年
    pub year:u32,
    ///月
    pub month:u32,
    ///日
    pub day:u32,
    ///時
    pub hour:u32,
    ///分
    pub minute:u32,
    ///店舗
    pub shop :u32,
    ///商品コード
    pub goods:String,
    ///バーコード
    pub barcode:String,
    ///割引コード
    pub discount:String,
    ///割引率
    pub rate:f32
} 

///実装
impl Sale{

    ///売り上げ登録
    pub fn insert(&self,tran:& mut Transaction)->Result<u32,Box<dyn Error>>
    {
        info!("insert start");
        let query=String::from(r"
        INSERT INTO t_sale 
        (year,month,day,hour,minute,shop,goods,barcode,discount,rate)
        VALUES
       (:year,:month,:day,:hour,:minute,:shop,:goods,:barcode,:discount,:rate)
        ");

        _=tran.exec_drop(query, 
            params!
            {
                "year"=>self.year,
                "month"=>self.month,
                "day"=>self.day,
                "hour"=>self.hour,
                "minute"=>self.minute,
                "shop"=>self.shop,
                "goods"=>self.goods.clone(),
                "barcode"=>self.barcode.clone(),
                "discount"=>self.discount.clone(),
                "rate"=>self.rate
            })?;
           
        let affwct_rows:u32=1;
        info!("insert end");
        Ok(affwct_rows)

    }
}


///コンストラクタ
pub fn from(year:&u32,
            month:&u32,
            day:&u32,
            shop:&u32,
            opencloselog:&locker::OpenCloseLog)->Result<Sale,Box<dyn Error>>
{
    let (hour,minute)=divide_clock(&opencloselog.open_time)?;
    let goods=opencloselog.get_goods_code();

    return Ok(Sale{
        id:0,
        year:*year,
        month:*month,
        day:*day,
        hour:hour,
        minute:minute,
        shop:*shop,
        goods:goods,
        barcode:opencloselog.open_barcode.clone(),
        discount:String::from("000"),
        rate:0.00
    });
}

///時間の分解
fn divide_clock(time:&str)->Result<(u32,u32),Box<dyn Error>>
{
   let buf:Vec<&str>= time.split(":").collect();

   let hour=buf[0].parse()?;
   let minute=buf[1].parse()?;

    return Ok((hour,minute));

}