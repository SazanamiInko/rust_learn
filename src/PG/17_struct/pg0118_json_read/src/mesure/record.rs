////////////////////////////
/// 
/// 計測データ構造体
/// 
/// /////////////////////////
use chrono::{Datelike, Local};
use serde::{Deserialize, Serialize};

///計測データ
#[derive(Serialize,Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Mesure
{
    ///計測年
    pub year:i32,
    ///計測月
    pub month:u32,
    ///計測日
    pub day:u32,
    ///血圧
    pub pressur:i32,
    ///体温
    pub temp:f32
}

///実装
impl Mesure{

    ///表示文字列取得
    pub fn get_display(&self)->String
    {
      return  format!("{}年{}月{}日,血圧{},体温{}"
                ,self.year
                ,self.month
                ,self.day
                ,self.pressur
                ,self.temp);

    }
}

///コンストラクタ
pub fn from(pressur:&i32,temp:&f32)->Mesure
{
    let date=Local::now();
   
    return Mesure
    {
        year:date.year(),
        month:date.month(),
        day:date.day(),
        pressur:*pressur,
        temp:*temp
    }; 
}