///////////////////////////////////////
/// 
/// 天候データAPI
/// 
/// ///////////////////////////////////
use super::components::data_models::weather_data_model::WeatherDataModel;

//最高気温を検索する
pub fn get_over_max_temp_weather(temp:f32)->Vec<WeatherDataModel>
{
        let mut ret:Vec<WeatherDataModel>=Vec::new();
        let resources=WeatherDataModel::load();
        let selected_list:Vec<_>=resources.iter()
                                 .filter(|record|record.max_temp>=temp)
                                .map(|data|data)
                                .collect();

        for record in selected_list
        {
                ret.push((*record).clone());
        }
        return ret;
}

///最低気温、最高気温を表示する
pub fn display_temp(datalist:&Vec<WeatherDataModel>)->()
{
        println!("{}件のデータがあります",datalist.len());
       
        for record in datalist
        {
                println!("{}/{}/{} 最高{} 最低{}",record.year
                                                ,record.month
                                                ,record.day
                                                ,record.max_temp
                                                ,record.mini_temp);
        }
        
}

