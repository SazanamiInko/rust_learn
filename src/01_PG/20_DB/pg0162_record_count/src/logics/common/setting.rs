//////////////////////////////////
/// 
/// 設定ファイル
/// 
/// ///////////////////////////////
use confy;
use log::info; 
use serde_derive::{Deserialize, Serialize};

///設定ファイル
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Setting{

    ///店舗番号
    pub shop:u32,
    ///サーバー情報,
    pub server:ServerInfo,
    ///ロッカー情報
    pub locker:LockerInfo

}

///サーバー情報
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct ServerInfo{
    ///ユーザー
    pub user:String,
    ///パスワード
    pub password:String,
    ///ホスト
    pub host:String,
    ///ポート番号
    pub port:String,
    ///使用DB
    pub db:String
}

//ロッカー情報
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct LockerInfo{

    ///開閉記録のパス
    pub path:String
}

///設定ファイルの読み込み
pub fn load()->Setting
{
   info!("load start");
   let  config= confy::load::<Setting>("pokapoka", "setting")
                                               .expect("設定ファイルの読み込みに失敗しました");
    info!("load end");
   return config;
}

