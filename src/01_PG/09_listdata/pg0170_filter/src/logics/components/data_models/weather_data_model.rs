///////////////////////////////////
/// 
/// 天候データ
/// 
/// ////////////////////////////////
use std::fs::File;
use csv;
use crate::logics::components::common::consts;

#[derive(Clone)]
//データ定義
pub struct WeatherDataModel
{
    //年
    pub year:u32,
    //月
    pub month:u32,
    //日
    pub day:u32,
    //雨量
    pub rain:f32,
    //最低気温
    pub mini_temp:f32,
    //最高気温
    pub max_temp:f32
}

//実装
impl WeatherDataModel
{
    //コンストラクタ
    pub fn new(year:u32,
               month:u32,
               day:u32,
               rain:f32,
               mini_temp:f32,
               max_temp:f32
            )->Self
    {
        return WeatherDataModel
        {
            year:year,
            month:month,
            day:day,
            rain:rain,
            max_temp:max_temp,
            mini_temp:mini_temp
        }
    }

    //CSVファイルの読み込み
    pub fn load()->Vec<WeatherDataModel>
    {
        let mut ret:Vec<WeatherDataModel>=Vec::new();

        let path:String=r"C:\Users\Public\Documents\data\Rust\pg0170\pg0170.csv".to_string();

        //開閉記録の読み込み
        let file=File::open(path).unwrap();
        let mut reader =  csv::ReaderBuilder::new()
                          .has_headers(false)
                          .from_reader(file);
      
        //開閉記録の格納
        for result in reader.records()
        {
          let record = result.unwrap();

          let year:u32=record[consts::CSV_YEAR_POS]
                        .parse()
                        .unwrap();

          let month:u32=record[consts::CSV_MONTH_POS]
                        .parse()
                        .unwrap();

          let day:u32=record[consts::CSV_DAY_POS]            
                      .parse()
                      .unwrap();

          let rain:f32=record[consts::CSV_RAIN]
                       .parse()
                       .unwrap();

          let mini_temp:f32=record[consts::CSV_MINI_TEMP]
                            .parse()
                            .unwrap();

          let max_temp:f32=record[consts::CSV_MAX_TEMP]
                           .parse()
                           .unwrap();

          let weather=WeatherDataModel::new(year
                                                            ,month
                                                            ,day
                                                            ,rain
                                                            ,mini_temp
                                                            ,max_temp);
          ret.push(weather);
        }
        return ret;
    }
}