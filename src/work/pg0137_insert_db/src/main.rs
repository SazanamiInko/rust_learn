//////////////////////////////////
/// 
/// DBへのデータ挿入
/// 
/// //////////////////////////////
mod logics;

use logics::common::setting;
use logics::sales_api;

///メイン関
fn main() {

 //設定ファイルの読み込み
 let config: setting::Setting=setting::load();
 //売り上げ登録
 let result= sales_api::add_uriage(&config);

 if let count=result
 {
    println!(”{}件の売り上げを登録しました",count);
 }


}
   
