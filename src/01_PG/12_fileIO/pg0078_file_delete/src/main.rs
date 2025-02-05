/////////////////////////////////
///
///ファイル削除
/// 
//////////////////////////////////
use std::fs::{self, remove_file};

//メイン関数
fn main() {
  //未処理から
  let path=r"C:\Users\Public\Documents\Data\Rust\未処理";
  let filelist=fs::read_dir(path).unwrap();

  for file_entry in filelist
  {
     let filename=file_entry.unwrap().path().to_str().unwrap().to_string();
     
     match remove_file(&filename){
     Ok(())=>{ println!("{}を削除しました",&filename);}
     Err(_e)=>{ println!("{}を削除できませんでした",&filename);}
     }
    
  }

}
