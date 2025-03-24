////////////////////////
/// 
/// 売上実績API
/// 
/// ////////////////////
use std::error::Error;
use rusqlite::Connection;
use rusqlite::params;
use rusqlite:: Result;
use crate::logics::components::common::util;
use crate::logics::components::datamodel::sale_datamodel::SaleDataModel;
use crate::logics::components::verifys::param_verifys::equal_verify::EqualVerify;
use crate::logics::components::verifys::param_verifys::range_verify::RangeVerify;
use crate::logics::components::common::const_list;
use super::components::verifys::param_verifys::ParamVerify;

//売上ファイルの作成
pub fn create_dbfile()->Result<(),Box<dyn Error>>
{
   let filename=util::get_file_name();
    let query=String::from(r"CREATE TABLE IF NOT EXISTS sales (
             id INTEGER PRIMARY KEY,
             take_on INTEGER NOT NULL,
             take_off INTEGER NOT NULL,
             price INTEGER NOT NULL,
             distance INTEGER NOT NULL,
             time TEXT  NOT NULL
         )");

     // SQLiteデータベースファイルに接続（なければ作成）
     let conn = Connection::open(filename)?;

     // テーブルの作成
     conn.execute(query.as_str(),[])?;
         
    return Ok(());
}

//売上実績の計上
pub fn add_sale_record(take_on:i32,take_off:i32)->Result<(),Box<dyn Error>>
{
    //引数検査
    let equal_verify=EqualVerify::set("引数チェック",
                                         "乗駅", 
                                         take_on, 
                                         "降駅", 
                                         take_off);

    let from_range_verify=RangeVerify::set("乗駅",
                                                 take_on,
                                                  const_list::NORTH_STATION,
                                                  const_list::SOUTH_STATION);

    let to_range_verify=RangeVerify::set("降駅",
                                              take_off,
                                               const_list::NORTH_STATION,
                                               const_list::SOUTH_STATION);
    //同値検査
    equal_verify.verify()?;

    //範囲検査
    from_range_verify.verify()?;
    to_range_verify.verify()?;

    //売上データの登録
    let filename=util::get_file_name();
    let datamodel=SaleDataModel::new(take_on, take_off);
    let mut conn = Connection::open(filename)?;
    let tran=conn.transaction()?;
   
    let query= r"INSERT INTO sales (take_on,take_off,price,distance,time) 
                        VALUES (?1, ?2,?3,?4,strftime('%H:%M', 'now', 'localtime'))"; 

    let add_result=tran.execute(query, params![datamodel.take_on,
                                    datamodel.take_off,
                                    datamodel.price,
                                    datamodel.distance])?;
    
    if add_result==0
    {
        _=tran.rollback();
        return Ok(())
    }

    _=tran.commit();
    //売上を登録する
    return Ok(());
}