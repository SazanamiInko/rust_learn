/////////////////////////////////
///
///フォーマット表示3(文字寄せ)
/// 
//////////////////////////////////

/// . メイン関数
fn main() {
    //変数
    let displayVal11=11;

    //表示
    println!("{:>8}",displayVal11);
    println!("{:^8}",displayVal11);
    println!("{:<8}",displayVal11);
}
