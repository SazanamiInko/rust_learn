/////////////////////////////////
/// 
/// 気温データの抽出
/// 
/// /////////////////////////////
mod logics;

use std::io;
use logics::weather_api::{self, display_temp, get_over_max_temp_weather};

///主処理
fn main() {

    let mut buf:String="".to_string();

    println!("熊谷市天気データから検索します。");
    println!("最高気温が何度以上のデータが欲しいですか?");
    _=io::stdin().read_line(&mut buf);
    
    let temp:f32=buf.trim()
                    .parse()
                    .expect("不正な値が入力されました");
    
    let find_data=get_over_max_temp_weather(temp);
    display_temp(&find_data);
}
