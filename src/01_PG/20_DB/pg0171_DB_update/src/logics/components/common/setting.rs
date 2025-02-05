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

///コンストラクタ
pub fn create( shop:u32,
    user:String,
    password:String,
    host:String,
    port:String,
    db:String,
    path:String)->Setting
{
    let server=ServerInfo{
        user:user,
        password:password,
        host:host,
        port:port,
        db:db
    };

    let locker=LockerInfo{path:path};

    return Setting{
        shop:shop,
        server:server,
        locker:locker
    }
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

///設定ファイルの書き込み
pub fn write(){
let setting=create(4,
    String::from("root"),
    String::from("p0kap0ka"),
    String::from("127.0.0.1"),
    String::from("3306"),
    String::from("uriage"),
    String::from(r"C:\Users\Public\Documents\Data\Rust\work\pokapoka\master_card_maint.log"));

    confy::store("pokapoka","setting", setting)
        .expect("設定ファイルの保存に失敗しました");
}