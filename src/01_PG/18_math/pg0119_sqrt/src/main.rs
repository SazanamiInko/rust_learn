//////////////////////
/// 
/// 平方根
/// 
/// ///////////////////

///メイン関数
fn main() {

    const TARGET:f64=17.0;
    let sqrt:f64=(TARGET).sqrt();
    let sqrt=(sqrt*100.00).round();
    let sqrt=sqrt/100.0;

    println!("{}の平方根は{:.2}です。",TARGET,sqrt);

}
