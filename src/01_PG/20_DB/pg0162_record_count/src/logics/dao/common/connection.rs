//////////////////////////////
/// 
/// 接続
///
/// //////////////////////////
use mysql::*;
use crate::logics::common::setting;
use log::info;

///接続情報
pub struct Connection
{
    pub user:String,
    pub password:String,
    pub host:String,
    pub port:String,
    pub db:String
}

///実装
impl Connection
{
    ///接続取得
    pub fn get_connection(&self)->PooledConn
    {

        info!("get_connection start");
        let url= format!("mysql://{}:{}@{}:{}/{}",
                        self.user,
                        self.password,
                        self.host,
                        self.port,
                        self.db);
        
        let pool = Pool::new(url.as_str()).expect("データベースへの接続に失敗しました");

        let conn = pool.get_conn().expect("接続の取得に失敗しました");
        info!("get_connection end"); 
        return conn;
    }
}

///コンストラクタ
pub fn from_config(server:&setting::ServerInfo)->Connection
{
    return Connection{
        user:server.user.clone(),
        password:server.password.clone(),
        host:server.host.clone(),
        port:server.port.clone(),
        db:server.db.clone()
    };

}