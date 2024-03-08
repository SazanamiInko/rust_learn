
////////////////////////////
/// 
/// 月間MKDir
/// 
/// ////////////////////////
use std::fs;
use std::io;
use chrono::{Datelike,NaiveDateTime, TimeDelta};

fn main() {

    let mut year_str="".to_string();
    let mut month_str="".to_string();
    let mut year:u32=0;
    let mut month:u32=0;

    println!("{}","年を入力してください");
    io::stdin().read_line(&mut year_str);
    println!("{}","月を入力してください");
    io::stdin().read_line(&mut month_str);

    //変換
    year=year_str.trim()
                    .parse()
                    .expect("年が不正です");

    month =month_str.trim()
        .parse()
        .expect("月が不正です");

    //月チェック
    if month<=0 || month>=13
    {
        println!("月”が不正です");
        return;
    }

    let first_day_val=format!("{}/{}/1 0:0:0",year,month);
    let mut work_date:NaiveDateTime=NaiveDateTime::parse_from_str(&first_day_val, "%Y/%m/%d %H:%M:%S").unwrap();

   
    //来月になるまで繰り返す
    loop {

        //ディレクトリを作成する
        let dirname=format!("{}月{}日",month,work_date.day());

       match fs::create_dir(dirname.to_string()) {
           Ok(())=>{}
           Err(e)=>{println!("{}の作成に失敗しました。",dirname);}
       }

        //1日加算
        work_date=work_date+TimeDelta::try_hours(24).unwrap();

        let month_step=work_date.month();

       if month_step!=month
       {
            break;
       }
    }
}