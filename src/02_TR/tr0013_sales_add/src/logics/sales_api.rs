/////////////////////////////////////////////
/// 
/// 売り上げAPI
/// 
/// /////////////////////////////////////////

use crate::logics::common::*;
use crate::logics::common::discount as discount_util;
use crate::logics::dao::*;
use crate::logics::dao::discount as discount_dao;
use mysql::*;
use log::info;
use std::error::Error;

///売り上げの登録
pub fn add_uriage(setting:&setting::Setting)->Result<u32,Box<dyn Error>>
{
    info!("add_uriage start");
    let mut cnt=0;
   
    //日はファイルの作成日（ロッカーは日ごとに開閉ログを作成）
    let (year,month,day)=locker::get_created_date(setting.locker.path.as_str());
    //開閉記録を読み込む
    let openlog=locker::load(setting.locker.path.as_str())?;
  
    //MySQLへの接続
    let connection_info=connection::from_config(&setting.server);
    let mut pool=connection_info.get_connection();
   
    //トランザクションの作成
    let mut tran =pool.start_transaction(TxOpts::default()).unwrap();

    //割引マスタの取得
    let discounts=discount_dao::get_time_discount(&mut tran)?;

    //トランザクションの開始
    for log in openlog
    {
       let mut sale_data= sale::from(&year,&month,&day,&setting.shop,&log)?;

      //適用する割引があるか?
      match discount_util::get_timesale(&sale_data.hour, &discounts) {
          //割引がある場合は割り引き情報を
          Some(target_discount)=>{discount_util::set_discount(&mut sale_data, &target_discount)}
          //割引がない場合は何もしない
          None=>{}
      }

       let row_count=sale_data.insert(&mut tran)?;
       cnt+=row_count;
    }

    //トランザクションをコミット
    _=tran.commit();
    info!("add_uriage end");
    return Ok(cnt);
}