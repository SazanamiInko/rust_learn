////////////////////////////////
/// 
/// ZIP圧縮
/// 
/// /////////////////////////////
mod logics;

///メイン関数
fn main() {

 //ZIP化するパス
 let source_path=r"C:\Users\Public\Documents\Data\Rust\work\pg0163\target";
 //ZIP出力するパス
 let dest_path=r"C:\Users\Public\Documents\Data\Rust\work\pg0163\out";
 
 println!("zip圧縮します");

  match logics::zip_api::compress_zip(source_path,dest_path)
  {
    Err(e)=>
    {
      println!("{}",e);
      println!("ZIP圧縮に失敗しました");
    }
    Ok(())=>
    {
      println!("zip圧縮しました");
    }
  }

}
