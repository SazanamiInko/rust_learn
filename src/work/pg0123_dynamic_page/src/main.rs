//////////////////////////////////////
/// 
/// 動的ページ
/// 
/// ///////////////////////////////////

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use tera::Tera;

#[actix_web::main]
///メイン関数
fn main() {
// テンプレートの設定
let tera = Tera::new("templates/**/*").unwrap();

HttpServer::new(move || {
    App::new()
        .app_data(web::Data::new(tera.clone()))
        .route("/", web::get().to(goods_list))
})
.bind("127.0.0.1:8080")?
.run()
.await

}

///商品マスタ
async fn goods_list()->impl Responder {

    let mut ctx = tera::Context::new();
    ctx.insert("name", "Rustacean");

    let rendered = tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)

}
