///////////////////////////////////////////
/// 
/// 設定ファイルAPI
/// 
/// ///////////////////////////////////////
use crate::logics::components::common::setting;

pub fn write_default()
{
    setting::write_default();
    println!("設定ファイルを書き込みました");
}