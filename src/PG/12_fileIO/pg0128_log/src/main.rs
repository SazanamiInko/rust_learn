///////////////////////////////////////
/// 
/// ログ
/// 
/// ///////////////////////////////////
mod logics;
use flexi_logger::{Logger, opt_format, FileSpec};
use log::{info, warn, error};
use std::env;

///メイン関数
fn main() {

    //ログの初期化
    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(FileSpec::default()
                                         .directory("logs")
                                         .basename("uriage"))
                                         .format(opt_format)
                                         .start()
                                         .unwrap();
   let mode =0;

    info!("バッチ処理開始");

   let args: Vec<String> = env::args().collect();
   let count=args.len()-1;

   if count!=1 
   {
        error!("引数が不正です。");
        return;
   }
   let mode=args[1].parse().unwrap();

   if mode==2
   {
        warn!("モード2は開発中です");
   }

    logics::uriage::up_uriage(&mode);

   info!("バッチ処理終了");

}
