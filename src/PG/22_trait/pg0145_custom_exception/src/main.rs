/////////////////////////////////
/// 
/// 独自の例外
/// 
/// //////////////////////////////
mod logics;
use logics::{traits::verify::Verify, verifys::range_verify};
///メイン関数
fn main() {      
 //範囲チェック
 let mut range_verify=range_verify::create(3,10);

 for i in 0..12
 {
    let label=format!("{}を検証",i);
    range_verify.set(label,i);

    match range_verify.verify() 
    {
        Ok(())=>{println!("{}はOK",i);},
        Err(e)=>{println!("{}",e);}
    }
 }

}
