//////////////////////////
/// 
/// TOP10の取得
/// 
/// //////////////////////
mod logics;

use crate::logics::components::common::consts;
use crate::logics::components::common::enums::SortType;
use crate::logics::weather_api::{display_temp, get_max_temp_weather, get_top10};

fn main() {
    
    //最高気温でソートする。
    let sorted_data_list=get_max_temp_weather(consts::MINI_NO_TEMP,
                                                                    consts::MAX_NO_TEMP, 
                                                                    SortType::Dsc);
    //TOP10を取得
    let top10=get_top10(&sorted_data_list);

    display_temp(&top10);                                                     
}
