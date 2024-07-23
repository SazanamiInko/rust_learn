////////////////////////////////
/// 
/// ZIP圧縮
/// 
/// /////////////////////////////
mod logics;

///メイン関数
fn main() {
 let path=r"C:\Users\Public\Documents\Data\Rust\work\pg0163\target";
 println!("zip圧縮します。");
  _=logics::zip_api::compress_zip(path);
  println!("zip圧縮しました");
}
