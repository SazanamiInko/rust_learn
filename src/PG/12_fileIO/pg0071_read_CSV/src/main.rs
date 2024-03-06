////////////////////////
/// 
/// CSVの分解
/// 
/// ////////////////////

//標準ライブラリ
use std::fs::File;
use csv;

//メイン関数
fn main() {
   readCSV();
}

//CSVファイルの読み込み
fn readCSV()
{
    let path=r"C:\Users\Public\Documents\Data\Rust\csv.txt".to_string();
    let file=File::open(path).unwrap();
    let mut reader =  csv::ReaderBuilder::new()
                      .has_headers(false)
                      .from_reader(file);
  
    let mut vecs:Vec<String>=Vec::new();

    for result in reader.records()
    {
      let record = result.unwrap();
        println!("{:?}", record);
    }

   
}

