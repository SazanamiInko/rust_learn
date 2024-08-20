//////////////////////////////
/// 
/// 接続
///
/// //////////////////////////
use mysql::*;
use crate::logics::components::common::setting;

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

        let url= format!("mysql://{}:{}@{}:{}/{}",
                        self.user,
                        self.password,
                        self.host,
                        self.port,
                        self.db);
        
        let pool = Pool::new(url.as_str()).expect("データベースへの接続に失敗しました");

        let conn = pool.get_conn().expect("接続の取得に失敗しました");
        return conn;
    }
}

///コンストラクタ
pub fn create()->Connection
{
    return Connection{
        user:String::from("root"),
        password:String::from("p1kap1ka"),
        host:String::from("127.0.0.1"),
        port:String::from("3306"),
        db:String::from("uriage")
    };
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