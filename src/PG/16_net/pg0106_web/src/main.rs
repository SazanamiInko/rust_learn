////////////////////////////////////////////
/// 
/// Web
/// 
/// ////////////////////////////////////////

use actix_web::{get, App, HttpServer, Responder};

//Web api
#[get("/")]
async fn index() -> impl Responder {
   "まめるり波"
}

//メイン関数
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}