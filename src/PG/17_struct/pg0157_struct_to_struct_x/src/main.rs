//////////////////////////////////
/// 
/// 構造体から構造体
/// 
/// ///////////////////////////////
mod logics;

///メイン関数
fn main() {
    let stocker=logics::stocker::Stocker::new("112233445566",2012,107);
    let benefit=logics::benefit::Benefit::new(&stocker);
}
