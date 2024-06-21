/////////////////////////////////////////////
/// 
/// 売り上げ
/// 
/// /////////////////////////////////////////

use crate::logics::common::*;
use crate::logics::dao::*;
use mysql::*;
use std::error::Error;

///売り上げの登録
pub fn add_uriage(setting:&setting::Setting)->Result<u32,Box<dyn Error>>
{
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

    //トランザクションの開始
    for log in openlog
    {
       let sale= sale::from(&year,&month,&day,&setting.shop,&log)?;
       let row_count=sale.insert(&mut tran)?;
       cnt+=row_count;
    }

    _=tran.commit();
    return Ok(cnt);
}