////////////////////////////
/// 
/// 1行ずつ読み込み
/// 
/// ////////////////////////
use std::fs::File;
use std::io::{BufRead, BufReader};

///主関数
fn main() 
{
   let path=r"C:\Users\Public\Documents\data\Rust\pg0182\branch_list.txt";
   let file=File::open(path)
                      .unwrap();

   let reader=BufReader::new(file);

   let branches:Vec<String>= reader.lines()
         .map(|record|record.unwrap())
         .collect();

   for branch in branches
   {
      println!("{}",branch);
   }


}
