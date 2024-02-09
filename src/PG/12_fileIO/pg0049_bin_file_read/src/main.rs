/////////////////////
/// 
/// バイナリ読み込み
/// 
/// /////////////////

//標準ライブラリ
use std::fs::File;
use std::io::prelude::*;

//メイン関数
fn main()->std::io::Result<()> {
 let mut readdata:[u8;4]=[0;4];
 let path=r"C:\Users\Public\Documents\Data\uriage.txt".to_string();
 let mut file =File::open(path)?;
 file.read_exact(&mut readdata);

 for readd in readdata
 {
    println!("{:02X}",readd);
 }

 Ok(())
}
