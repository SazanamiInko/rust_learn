///////////////////////////////////////
/// 
/// 天候データAPI
/// 
/// ///////////////////////////////////
use super::components::data_models::weather_data_model::WeatherDataModel;
use crate::logics::components::common::enums::SortType;
use std::cmp::Ordering;

//最高気温を検索する
pub fn get_max_temp_weather(low_temp:f32,high_temp:f32,sort:SortType)->Vec<WeatherDataModel>
{
        let mut ret:Vec<WeatherDataModel>=Vec::new();
        let resources=WeatherDataModel::load();
        let selected_list:Vec<_>=resources.iter()
                                 .filter(|record|record.max_temp>=low_temp)
                                 .filter(|record|record.max_temp<=high_temp)
                                 .map(|data|data)
                                .collect();

        for record in selected_list
        {
                ret.push((*record).clone());
        }

        if ret.len()>1
        {
                //並び替え
                match sort {

                        SortType::Asc=>
                        {
                                //昇順
                                ret.sort_by(|a,b|
                                                     a.max_temp.partial_cmp(&b.max_temp)
                                                     .unwrap_or(Ordering::Equal));
                        },
                        SortType::Dsc=>
                        {
                                //降順
                                ret.sort_by(|a,b|
                                        b.max_temp.partial_cmp(&a.max_temp)
                                        .unwrap_or(Ordering::Equal));

                        },
                        SortType::NoSort=>
                        {
                                //何もしない
                        }           
                }
        }


        return ret;
}

///最低気温を検索する
pub fn get_mini_temp_weather(low_temp:f32,high_temp:f32,sort:SortType)->Vec<WeatherDataModel>
{
        let mut ret:Vec<WeatherDataModel>=Vec::new();
        let resources=WeatherDataModel::load();
        let selected_list:Vec<_>=resources.iter()
                                 .filter(|record|record.mini_temp<=low_temp)
                                 .filter(|record|record.mini_temp>=high_temp)
                                 .map(|data|data)
                                .collect();

        for record in selected_list
        {
                ret.push((*record).clone());
        }

        if ret.len()>1
        {
                //並び替え
                match sort {

                        SortType::Asc=>
                        {
                                //昇順
                                ret.sort_by(|a,b|
                                                     a.mini_temp.partial_cmp(&b.mini_temp)
                                                     .unwrap_or(Ordering::Equal));
                        },
                        SortType::Dsc=>
                        {
                                //降順
                                ret.sort_by(|a,b|
                                        b.mini_temp.partial_cmp(&a.mini_temp)
                                        .unwrap_or(Ordering::Equal));

                        },
                        SortType::NoSort=>
                        {
                                //何もしない
                        }           
                }
        }
        
        return ret;
}

///TOP10を取得
pub fn get_top10(datalist:&Vec<WeatherDataModel>)->Vec<WeatherDataModel>
{
        let top10=datalist.iter()
        .take(10);
        
        let mut ret:Vec<WeatherDataModel>=Vec::new();

        for record in top10
        {
                ret.push(record.clone());
        }

        return ret;
}


///最低気温、最高気温を表示する
pub fn display_temp(datalist:&Vec<WeatherDataModel>)->()
{
        println!("{}件のデータがあります",datalist.len());
       
        for record in datalist
        {
                println!("{}/{}/{} 最高{:.1} 最低{:.1}"
                                                ,record.year
                                                ,record.month
                                                ,record.day
                                                ,record.max_temp
                                                ,record.mini_temp);
        }
        
}

