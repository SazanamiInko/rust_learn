/////////////////////////////////////////
/// 
/// 無名関数の所有権移動
/// 
/// ////////////////////////////////////
use std::thread;

///メイン関数
fn main() {

    println!("ここは神の世界");
    let kaasuke=String::from("カー助");

    let handle = thread::spawn( || {
        println!("恋愛下手なカー子が泣いてる人の世");
        println!("{}参上",kaasuke);
    });

    // 新しいスレッドが完了するのを待つ
    handle.join().unwrap();

}
