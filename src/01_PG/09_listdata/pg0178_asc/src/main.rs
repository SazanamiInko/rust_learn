////////////////////////////////////////////
/// 
/// データ並び替え
/// 
/// ////////////////////////////////////////
mod logics;

use crate::logics::weather_api;
use crate::logics::components::common::consts;
use  crate::logics::components::common::enums::SortType;

///主関数
fn main() {
 let result=   weather_api::get_mini_temp_weather(0.0,
                                      consts::MINI_NO_TEMP, 
                                      SortType::Asc);

  weather_api::display_temp(&result);                                    
}
