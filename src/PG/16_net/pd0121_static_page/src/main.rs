//////////////////////////
/// 
/// 静的ページ表示
/// 
/// ///////////////////////
mod logics;
use warp::Filter;

#[tokio::main]
async fn main() {

      // warpフィルターを使ってHTMLを返すエンドポイントを作成
     let route = warp::path::end()
     .map(move || {warp::reply::html(logics::file_io::get_index_page())});
 
     // サーバーを起動
     warp::serve(route)
         .run(([127, 0, 0, 1], 3030))
         .await;
}
