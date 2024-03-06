/////////////////////////
///
/// CSVの作成
///
/////////////////////////


use chrono::{ Datelike, NaiveDateTime,Utc,DateTime};
use std::io::Write;
use std::fs::OpenOptions;

//メイン関数
fn main() {
   let csv=makeCSV();
   write_record(csv); 
}

//CSVの作成
fn makeCSV()->String
{
    //現在時刻を取得
let ima=get_now();
    //文字を取得
   let csv=format!("{},{},{}",
                   ima.year(),
                   ima.month(),
                   ima.day());
   return csv;
}

//レコードを保存
fn write_record(record:String)
{
 let path=r"C:\Users\Public\Documents\Data\Rust\csv.txt";
 let mut file =OpenOptions::new()
    .append(true)
    .open(path).unwrap();

    _=file.write(record.as_bytes());
    _=file.write("\n".as_bytes());
}

//日時を取得
fn get_now()->NaiveDateTime
{
    let utc_ima: DateTime<Utc>= Utc::now();
    let native_ima=utc_ima.naive_local();
    return  native_ima;
}
