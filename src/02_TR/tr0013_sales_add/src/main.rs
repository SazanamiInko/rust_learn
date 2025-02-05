//////////////////////////////////
/// 
/// DBへのデータ挿入
/// 
/// //////////////////////////////
mod logics;

use flexi_logger::{Logger, opt_format, FileSpec};
use log::{info, error};
use logics::common::setting;
use logics::sales_api;

///メイン関数
fn main() {

   //ログの初期化
   Logger::try_with_str("info")
   .unwrap()
   .log_to_file(FileSpec::default()
                                    .directory("logs")
                                    .basename("pg0137"))
                                    .format(opt_format)
                                    .start()
                                    .unwrap();

info!("売り上げ登録バッチ処理を開始しました");

 //設定ファイルの読み込み
 let config: setting::Setting=setting::load();
 //売り上げ登録
 let result= sales_api::add_uriage(&config);

 match result
 {
    Ok(cnt)=>
    {
        println!("{}件の売り上げを登録しました",cnt);
        info!("{}件の売り上げを登録しました",cnt);
        info!("売り上げ登録バッチ処理を正常終了しました")
    },
    Err(e)=>
    {
        println!("売り上げ登録に失敗しました");
        info!("売り上げ登録バッチ処理を異常終了しました");
        error!("{}",e);
    }
 }

}
   
