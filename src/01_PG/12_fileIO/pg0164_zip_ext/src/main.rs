/////////////////////////////////////////
/// 
/// ZIP展開
/// 
/////////////////////////////////////////

mod logics;

///メイン関数
fn main() {
    //ZIP化するパス
 let source_path=r"C:\Users\Public\Documents\Data\Rust\work\pg0164\target.zip";
 //ZIP出力するパス
 let dest_path=r"C:\Users\Public\Documents\Data\Rust\work\pg0164\out";
 
 println!("zip展開します");

  match logics::zip_api::extract_zip(source_path, dest_path)
  {
    Err(e)=>
    {
      println!("{}",e);
      println!("ZIP展開に失敗しました");
    }
    Ok(())=>
    {
      println!("zip展開しました");
    }
  }
}
