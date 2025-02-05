////////////////////////////
/// 
/// 文字列の分解
/// 
/// ////////////////////////

//メイン関数
fn main() {
   let fullname="image.png".to_string();
   let dev:Vec<_>=fullname.split('.').collect();

    let filename=dev[0];
    let extend=dev[1];

    println!("フルネーム:{}",fullname);
    println!("ファイル名:{}",filename);
    println!("拡張子:{}",extend);
}
