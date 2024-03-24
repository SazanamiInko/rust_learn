/////////////////////////////////
///
/// and演算
/// 
//////////////////////////////////

//利用標準ライブラリ
use std::io;

//メイン関数
fn main() {
    let mut n=9;
   
    read_std_in(&mut n);


    if n<1||n>9
    {       
        println!("{}はデータが範囲外です",n);
        return;
    }
    println!("モード{}で運転します",n);
}

// 標準入力を行います
/// *message` - メッセージ
/// *in_param` - パラメータ
fn read_std_in(in_param:&mut i32)
{
    let mut buff:String=String::new();
 
    println!("運転モードを入力してください");

    io::stdin()
        .read_line(&mut buff)
        .expect("入力でエラーが発生しました");
 
    *in_param= buff.trim().parse().expect("入力でエラーが発生しました");
}