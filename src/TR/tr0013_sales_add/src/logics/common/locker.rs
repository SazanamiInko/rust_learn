/////////////////////////////////////
/// 
/// ロッカーモジュール
/// 
/// /////////////////////////////////

///定数
const START:usize=8;
const END:usize=12;

use chrono::prelude::{DateTime, Utc, Datelike };
use csv::Error;
use log::info; 
use std::fs;
use std::fs::File;

///開閉記録
pub struct OpenCloseLog
{
    ///ロッカー番号
    pub locker_no:String,
    ///ロッカー開扉時間
    pub open_time:String,
    ///ロッカー開扉時のバーコード
    pub open_barcode:String,
    ///ロッカー閉扉時間
    pub close_time:String,
    ///ロッカー閉扉時のバーコード
    pub close_barcode:String,
    ///開閉Ferica
    pub ferica_no:String,
    ///メンテフラグ
    pub is_mantain:String
}

///実装
impl OpenCloseLog{

    ///商品コードの取得
    pub fn get_goods_code(&self)->String
    {
        info!("get_goods_code start");
        let goods_code=self.open_barcode[START..END].to_string();
        info!("get_goods_code end");
        return goods_code; 
    }
}

///コンストラクタ
fn create(locker_no:String,
    open_time:String,
    open_barcode:String,
    close_time:String,
    close_barcode:String,
    ferica_no:String,
    is_mantain:String

    )->OpenCloseLog
    {
        return OpenCloseLog{
            locker_no:locker_no,
            open_time:open_time,
            open_barcode:open_barcode,
            close_time:close_time,
            close_barcode:close_barcode,
            ferica_no:ferica_no,
            is_mantain:is_mantain
        };
    }

///開閉記録の読み込み
pub fn load(path:&str)->Result<Vec<OpenCloseLog>,Error>
{
    info!("load start");
    let mut ret:Vec<OpenCloseLog>=Vec::new();

    //開閉記録の読み込み
    let file=File::open(path).unwrap();
    let mut reader =  csv::ReaderBuilder::new()
                      .has_headers(false)
                      .from_reader(file);
  
    //開閉記録の格納
    for result in reader.records()
    {
      let record = result.unwrap();
      let log_line=create(record[0].to_string(),
                         record[1].to_string(), 
                     record[2].to_string(),
                     record[3].to_string(), 
                     record[4].to_string(),
                     record[5].to_string(),
                     record[6].to_string()
                    );
     ret.push(log_line);
    }
    info!("load end");
    return Ok(ret);
}

///開閉ログの作成日取得
pub fn get_created_date(path:&str)->(u32,u32,u32)
{
    info!("get_created_date start");
    let meta= fs::metadata(path).unwrap();

    let created= meta.created().unwrap();
    let datetime_created_utc : DateTime<Utc>=created.into();

    info!("get_created_date end");
    
    return (datetime_created_utc.year() as u32
            ,datetime_created_utc.month()
            ,datetime_created_utc.day());
}