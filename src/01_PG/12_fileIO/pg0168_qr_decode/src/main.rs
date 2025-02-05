/////////////////////////////////////////
/// 
/// QRコードの読みこみ
/// 
/// /////////////////////////////////////
mod logics;

///メイン関数
fn main() {
    let path=r"C:\Users\Public\Documents\Data\Rust\work\pg0168\qr.png";

    match logics::qr_api::decode(path) {
        Ok(code)=>{println!("QRコードの内容:{}",code);}
        Err(e)=>
        {
            println!("{}",e);
            println!("QRコードのデコードに失敗しました。");
        }
        
    }
}
