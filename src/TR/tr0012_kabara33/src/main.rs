/////////////////////////////////////////////////////////
/// 
/// 数秘術33の列挙
/// 
/// ///////////////////////////////////////////////////////
use chrono::{Duration, NaiveDate};

///メイン関数
fn main() {
    //開始日
    let start_date = NaiveDate::from_ymd_opt(1975, 1, 1).unwrap();

    //終了日
    let end_date = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
    
    let mut current = start_date;
    let mut result = vec![];

    while current<=end_date {

        let kabara=get_kabara_from_date(&current);

        if kabara==33
        {
            result.push(current);
        }

        current=current+Duration::days(1);
    }

    println!("{}～{}の数秘術33となる日は{}日ありました",1975,2025,result.len());
    for date in result
    {
        println!("{}",date);
    }

}

///日付型からカバラ数の取得
fn get_kabara_from_date(date:&NaiveDate)->i32
{
    let str=date.to_string();
    let fomated_date:String=str.chars()
            .filter(|&char_data|char_data.to_string()!="-")
                 .collect();
    let num=fomated_date.parse()
                             .expect("変換に失敗しました");

    return get_kabara(&num);
}

///カバラ数ぬ取得
fn get_kabara(num:&i32)->i32
{
    let mut kabara=0;
    let mut work=num.clone();

    while work>0
    {
        kabara+=work%10;
        work=work/10;
    }

    if kabara>10
    {
        if kabara%10==kabara/10
        {
            return kabara;
        }
        else {
            return get_kabara(&kabara);
        }
    }

    return kabara;
}
