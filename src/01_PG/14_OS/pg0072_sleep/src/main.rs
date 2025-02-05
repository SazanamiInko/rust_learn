/////////////////////////
///
/// 停止
///
/////////////////////////
use std::thread;
use std::time::Duration;
use chrono::{NaiveDateTime,Utc,DateTime};

//メイン関数
fn main() {
    let ima: NaiveDateTime =get_naitive_now();
    println!("{}",ima);

    thread::sleep(Duration::from_millis(3*60*1000));

    let ato: NaiveDateTime =get_naitive_now();
    println!("{}",ato);
}

//現在時刻を取得
fn get_naitive_now()->NaiveDateTime
{
    let utc_ima: DateTime<Utc>= Utc::now();
    let native_ima=utc_ima.naive_local();
    return  native_ima;
}