/////////////////////////////////
///
/// 空文字判定
/// 
//////////////////////////////////

use std::io;

//メイン関数
fn main() {
    let mut work= String::from("");

    println!("入力してください");

    io::stdin().read_line(&mut work).expect("入力でエラーが発生しました");


    //改行文字を除去
    work=work.replace("\r\n", "");


    if work.is_empty()
    {
        println!("お願いいたします。入力してください");        
    }
    else
    {
        println!("ありがとうございます。");
    }

}
