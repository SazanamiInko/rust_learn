/////////////////////////
///
/// 期間の計算
///
/////////////////////////
//参考サイト
//https://qiita.com/someone7140/items/dc553e070a88862cd027
//

use chrono::{ Datelike, NaiveDateTime, TimeDelta,Utc,DateTime};


//メイン関数
fn main() {
    let ima: NaiveDateTime =get_naitive_now();
    let syougatu: NaiveDateTime = get_shougatu(&ima);
    let atoikutu=syougatu-ima;

    println!("今->{}",ima);
    report(atoikutu);
    
}

//現在の時刻を取得する
fn get_naitive_now()->NaiveDateTime
{
    let utc_ima: DateTime<Utc>= Utc::now();
    let native_ima=utc_ima.naive_local();
    return  native_ima;
}

//お正月を求める
fn get_shougatu(ima:&NaiveDateTime)->NaiveDateTime
{
    let year=ima.year()+1;
    let fmt="%Y/%m/%d %H:%M:%S";
 
    let val=format!("{}/1/1 0:0:0",year);
    let shougatu= NaiveDateTime::parse_from_str(&val, fmt)
    .unwrap();
    return shougatu;
}



//報告
fn report(atoikutu:TimeDelta)
{
    println!("あと{}寝るとお正月",atoikutu.num_days());
}