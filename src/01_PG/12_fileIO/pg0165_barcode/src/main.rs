///////////////////////////////////////////////////////
///
/// バーコード発行
///
///////////////////////////////////////////////////////
mod logics;

///メイン関数
fn main() {

 let barcode_data="HELLOHELLO";
 let path=r"C:\Users\Public\Documents\Data\Rust\work\pg0165\barcode.png";

    println!("バーコードを発行します");

    let code=logics::barcode_api::make_a_mode_data(barcode_data);
   match  logics::barcode_api::publish_barcode(code.as_str(), path) {

    Ok(())=>{println!("バーコードを発行しました。");}
    Err(e)=>{
        println!("{}",e);
        println!("バーコードを発行に失敗しました。");
    }
   }

}
