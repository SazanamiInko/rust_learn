/////////////////////////////////////////////////////////////////////////////////
/// 
/// バーコード読み取り
/// 
/// ////////////////////////////////////////////////////////////////////////////
mod logics;

///メイン関数
fn main() {
    let path=r"C:\Users\Public\Documents\Data\Rust\work\pg0166\barcode.png";
    println!("バーコードを読み取ります");
   match logics::barcode_api::read_barcode(path)
   {
    Ok(message)=>{println!("{}",message);},
    Err(e)=>{println!("{}",e);}
   }
}
