///////////////////////////////////
/// 
/// ファイルの有無検査
/// 
///////////////////////////////////

use std::path::Path;

fn is_exists(path:&str)->bool
{
   return Path::new(path).exists();
}


///主関数
fn main() {
   //引数チェック
   let path1=r"C:\Users\Public\Documents\data\Rust\pg0192\aaa.txt";
   let path2=r"C:\Users\Public\Documents\data\Rust\pg0192\aaab.txt";

   let mut pathes:Vec<String>=Vec::new();
   pathes.push(path1.to_string());
   pathes.push(path2.to_string());
   
   for path in pathes
   {
      println!("パス:{}",path);
      if is_exists(&path)
      {
         println!("ファイルあります");
      }
      else
      {
         println!("ファイルがありません");
      }
   }
}
