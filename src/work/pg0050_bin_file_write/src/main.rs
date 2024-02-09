////////////////////////////
/// 
/// バイナリ書き込み
/// 
/// ////////////////////////
//標準ライブラリ
use std::io::{self, Write};
use std::io::Read;
use std::fs::File;

///メイン関数
fn main() ->std::io::Result<()>{
let write_data:[u8;4]=[0x01,0x02,0x64,0x17];
let path=r"C:\Users\Public\Documents\Data\uriage.txt".to_string();
let mut f = File::create(path)?;

f.write_all(&write_data);
  Ok(())
}
