use logics::api;

//////////////////////////////////////////////////
/// 
/// レコード件数
/// 
/// ///////////////////////////////////////////////
mod logics;
use flexi_logger::{Logger, opt_format, FileSpec};



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

    let count=api::get_master_card()
    .unwrap();

    println!("カードが{}枚登録されてます",count)
}
