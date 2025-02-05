///////////////////////////
/// 
/// 動的HTML
/// 
/// /////////////////////////
use tera::{Context, Tera};
use warp::Filter;
use std::sync::Arc;
use chrono::Local;

#[tokio::main]
///メイン関数
async fn main() {
    //テンプレートの設定
    let tera = Arc::new(Tera::new("templates/**/*").unwrap());
    let tera_filter = warp::any().map(move || tera.clone());
    let date=Local::now();
    let date_str=date.to_string();
    let service = warp::path::end()
        .and(tera_filter)
        .map(move |tera: Arc<Tera>| {
            let mut context = Context::new();
            context.insert("time", date_str.as_str());
            let rendered = tera.render("index.html", &context).unwrap();
            warp::reply::html(rendered)
        });

         // サーバーの起動
    warp::serve(service).run(([127, 0, 0, 1], 3030)).await;
}
