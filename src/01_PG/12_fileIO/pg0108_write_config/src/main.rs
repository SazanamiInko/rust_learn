/////////////////////////////////
///
/// 設定ファイルの書き込み
/// 
//////////////////////////////////
use confy;
use confy::ConfyError;
use serde_derive::{Deserialize, Serialize};


///設定ファイル
#[derive(Default, Serialize, Deserialize, Debug)]
struct TConfig{
    version:i32,
    username:String,
    password:String
}

///メイン関数
fn main() {

    let conf=TConfig{
        version:1,
        username:String::from("user"),
        password:String::from("himitu")
    };

  match write_config(&conf) {
      Ok(())=>println!("設定ファイルの保存に成功しました"),
      Err(message)=>println!("{}",message),
  }

}

///設定ファイルの書き込み
fn write_config(conf:&TConfig)->Result<(),ConfyError>
{
    confy::store("pg0108","setting", conf)?;
    return Ok(());
}