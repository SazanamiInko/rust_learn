use std::path;

/////////////////////////////////////////////
/// 
/// 売り上げ
/// 
/// /////////////////////////////////////////
mod locker;

use super::setting::Setting;

///売り上げの登録
pub fn insert_uriage(setting:Setting)->Result<i32,String>
{
    let mut cnt=0;

   
    //開閉記録を読み込む
    let openlog=locker::load(setting.locker.path.as_str());


    return Ok((cnt));
}