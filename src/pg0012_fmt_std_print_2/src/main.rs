/////////////////////////////////
///
///フォーマット表示2(桁埋め、0埋め)
/// 
//////////////////////////////////

/// . メイン関数
fn main() {

    //変数
    let displayVal11=11;

    //表示
    println!("{:>8}",displayVal11);
    println!("{:0>8}",displayVal11);
    println!("{:?>8}",displayVal11);

    println!("{:>8b}",displayVal11);
    println!("{:0>8b}",displayVal11);
    println!("{:?>8b}",displayVal11);
}
