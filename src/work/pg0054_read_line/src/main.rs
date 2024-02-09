////////////////////////
/// 
/// 行読み込み
/// 
/// ////////////////////

//標準ライブラリ
use std::fs::File;
use std::io::{BufRead, BufReader};

//メイン関数
fn main()->std::io::Result<() >{
    let path=r"C:\Users\Public\Documents\Data\log.txt".to_string();
    let file=File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines()
    {
      println!("{}",line?);
    }

    Ok(())
}
