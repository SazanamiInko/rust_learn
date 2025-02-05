/////////////////////////////////////////
/// 
/// CSVの作成
/// 
/// //////////////////////////////////////

///メイン関数
fn main() {
    
    let gyou1=String::from("かささぎの");
    let gyou2=String::from("渡せる橋に");
    let gyou3=String::from("置く霜の");
    let gyou4=String::from("白きを見れば");
    let gyou5=String::from("白きを見れば");

    let utavec:Vec<String>=vec![gyou1,gyou2,gyou3,gyou4,gyou5];

    println!("{}",utavec.join(","))
}

