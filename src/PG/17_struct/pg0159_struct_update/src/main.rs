//////////////////////////////////
/// 
/// 構造体の更新
/// 
/// ///////////////////////////////
mod logics;

///メイン関数
fn main() {
    let stocker=logics::stocker::Stocker::new("112233445566",2012,107);
    let mut benefit=logics::benefit::Benefit::new(&stocker);
    println!("直後{}",benefit.benefit);
    logics::api::decide_benegit(&mut benefit);
    println!("更新{}",benefit.benefit);
}