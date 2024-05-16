mod logics;

pub mod salally
{
   
    ///給与を表す構造体
    pub struct Salally
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
        pub achievements:f32,
        ///給与原泉
        pub source:i32,														
        ///税
        pub  tax:i32,
        ///インボイス
        pub invoice:i32,
        ///控除後給与
        pub  amount:i32
    }

    ///給与を作成する
    ///
    /// args
    /// 
    /// no:社員番号
    /// start:始業時間
    /// end:終業時間
    /// 
    /// return
    /// 
    /// 給与
    /// 
    /// panic
    /// 
    /// 時間に数値が変換されないとき
    pub fn create_salally(no:String,start:String,end:String)->Result<Salally,String>{
        //時間の分解
        let (start_hour,start_minute)=conv_clock_hour(&start);
        let (end_hour,end_minute)=conv_clock_hour(&end);

        //実績の計算
        let start_unit=calc_achievements(start_hour,start_minute);
        let end_unit=calc_achievements(end_hour,end_minute);
        let achievements:f32=end_unit-start_unit;

        //開始時間と終了時間
        if start_unit>=end_unit
        {
            Err("時間がおかしいです");
        }
    
        let source=logic::get_salally(achievements);
        let tax=logic::get_tax(source);
        let invoice=logic::get_invoice(source);


        return Ok( Salally
        {
            no:no,
            start_hour:start_hour,
            start_minute:start_minute,
            end_hour:end_hour,
            end_minute:end_minute,
            achievements:achievements,
            source:ource,
            tax:tax,
            invoice:invoice,
            amount:source-(tax+invoice)
        });
    }


    ///文字列の時間を[時,分]に分解する
    pub fn conv_clock_hour(time_str:&str)->(i32,i32)
    {
        let time_string=time_str.to_string();
        let res: Vec<&str>=time_string.split(":").collect();

        let hour=res[0].trim().parse().expect("数値の変換に失敗しました");
        let minute=res[1].trim().parse().expect("数値の変換に失敗しました");
        return (hour,minute);
    }

    ///実績単位に変換
    pub fn calc_achievements(hour:i32,minute:i32)->f32
    {
        let hour_unit=hour as f32;
        let minute_multi=minute/15;
        let minute_unit=minute_multi as f32*0.25;
        return hour_unit+minute_unit;
    }
}