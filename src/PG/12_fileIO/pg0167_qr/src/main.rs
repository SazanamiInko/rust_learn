//////////////////////////////////////////////
/// 
/// QRコードを発行する
/// 
/// //////////////////////////////////////////
mod logics;

///メイン関数
fn main() {
    let path=r"C:\Users\Public\Documents\Data\Rust\work\pg0167\qr.png";
    let message="https://www.rogers.co.jp/shop/kitamoto/";

   match logics::qr_api::publish_qr(path, message)
   {
        Ok(())=>{println!("QRコードを発行しました。");}
        Err(e)=>{println!("{}",e);}
   }
}
