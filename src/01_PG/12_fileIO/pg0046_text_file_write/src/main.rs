////////////////////////////////////
/// 
/// ファイルの書き込み
/// 
////////////////////////////////////

//標準ライブラリ
use std::io::{self, Write};
use std::io::Read;
use std::fs::File;

///メイン関数 
fn main()->std::io::Result<()> {
   
   let mut count=get_count()?;
   report(count);
   write_Count(count);
   Ok(())
}

///アクセスカウンター取得
fn get_count() -> Result<i32, io::Error> {
    let path=r"C:\Users\Public\Documents\Data\access.txt".to_string();
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    let count:i32=s.trim().parse().unwrap();
    Ok(count+1)
}

//アクセスカウンター更新
fn write_Count(count:i32)->Result<bool,io::Error> {
    let path=r"C:\Users\Public\Documents\Data\access.txt".to_string();
    let mut f = File::create(path)?;
    let mut s = count.to_string();
    let mut bytes=s.as_bytes();
    f.write_all(bytes);
    Ok(true)
}

//表示
fn report(count:i32)
{
    println!("いらっしゃいませ");
    println!("あなたは{}人目のお客様です。",count);
}