///////////////////////////////
/// 
/// 設定ファイル
/// 
/// ///////////////////////////
use confy::ConfyError;
use serde_derive::{Deserialize, Serialize};

///設定ファイル構造体
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct TConfig{
    ///バージョン
    version:i32,
    ///ユーザー名
    username:String,
    ///パスワード
    password:String
}

///実装
impl TConfig{
    

    ///設定ファイルの書き込み
    pub fn write_config(&self)->Result<(),ConfyError>
    {
        confy::store("pg0108","setting", self)?;
        return Ok(());
    }

   ///表示
   pub fn display(&self)
   {
        println!("version={},username={},password={}",self.version,self.username,self.password);
   }
}

///コンストラクタ
pub fn create()->TConfig
{
    return TConfig{
        version:1,
        username:"user".to_string(),
        password:"himitsu".to_string()
    };
}

 ///設定ファイルの読み込み
 pub fn read_config()->Result<TConfig,ConfyError>
 {
    let  config= confy::load::<TConfig>("pg0108", "setting")?;
   
    return Ok(config); 
 }
