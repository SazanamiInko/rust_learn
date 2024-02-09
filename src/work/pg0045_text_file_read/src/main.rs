///////////////////////////////
/// 
/// ファイル読み込み
/// 
/// ////////////////////////////

//標準ライブラリ
use std::fs::File;
use std::io::prelude::*;

//メイン関数
fn main()->std::io::Result<()> {

 let path=r"C:\Users\Public\Documents\Data\access.txt".to_string();
 let mut file =File::open(path)?;
 let mut countStr :String=String::new();
 let mut count :i32=0;
 
 file.read_to_string(&mut countStr);
 
 count=countStr.trim().parse().unwrap();
 println!("あなたは{}人目のお客様です",count+1);

 Ok(())

}

