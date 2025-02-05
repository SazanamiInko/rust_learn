//////////////////////////////////////
/// 
/// 多重フィルタ
/// 
/// /////////////////////////////////
mod logics;
use logics::weather_api;
use std::io;

///主関数
fn main() {
    let mut buf1:String="".to_string();
    let mut buf2:String="".to_string();

    println!("熊谷市天気データから検索します。");
    println!("最高気温が何度以上のデータが欲しいですか?");
    _=io::stdin().read_line(&mut buf1);
    
    let low_temp:f32=buf1.trim()
                    .parse()
                    .expect("不正な値が入力されました");
   
   println!("最高気温が何度以下のデータが欲しいですか?");
                _=io::stdin().read_line(&mut buf2);
                
                let high_temp:f32=buf2.trim()
                                .parse()
                                .expect("不正な値が入力されました");

    let find_data=weather_api::get_over_max_temp_weather(low_temp,high_temp);
    weather_api::display_temp(&find_data);

}
