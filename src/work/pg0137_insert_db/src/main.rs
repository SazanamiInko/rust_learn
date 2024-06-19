
//////////////////////////////////
/// 
/// DBへのデータ挿入
/// 
/// //////////////////////////////
mod logics;

use logics::setting;
use logics::uriage;

///メイン関
fn main() {

 //設定ファイルの読み込み
 let config: setting::Setting=setting::load();
 let result= uriage::insert_uriage(config);


}
   
