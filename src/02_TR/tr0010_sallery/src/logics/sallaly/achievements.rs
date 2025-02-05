/////////////////////////////
/// 
/// 作業実績
/// 
/// //////////////////////////

///作業実績構造体
pub struct Achievements
{
  ///社員番号
  pub no:String,
  ///実績開始時間
  pub start_hour:i32,
  ///実績開始分
  pub start_minute:i32,
  ///実績終了時間
  pub end_hour:i32,																												
  ///実績終了分
  pub end_minute:i32,													
  ///実績単位
  pub achievements:f32
}

///コンストラクタ
pub fn from(no:&str,
            start:&str,
            end:&str)->Achievements
{

  let (start_hour,start_minute)=conv_clock_hour(start);
  let (end_hour,end_minute)=conv_clock_hour(end);
  let start_unit=calc_achievements(start_hour, start_minute);
  let end_unit=calc_achievements(end_hour, end_minute);

  return  Achievements
  {
    no:no.to_string(),
    start_hour:start_hour,
    start_minute:start_minute,
    end_hour:end_hour,
    end_minute:end_minute,
    achievements:end_unit-start_unit
  };

}

fn conv_clock_hour(time_str:&str)->(i32,i32)
 {
     let time_string=time_str.to_string();
     let res: Vec<&str>=time_string.split(":").collect();

     let hour=res[0].trim().parse().expect("数値の変換に失敗しました");
     let minute=res[1].trim().parse().expect("数値の変換に失敗しました");
     return (hour,minute);
 }

 fn calc_achievements(hour:i32,minute:i32)->f32
 {
     let hour_unit=hour as f32;
     let minute_multi=minute/15;
     let minute_unit=minute_multi as f32*0.25;
     return hour_unit+minute_unit;
 }