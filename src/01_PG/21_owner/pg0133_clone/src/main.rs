
/////////////////////////////////////////
/// 
/// クローン
/// 
/// ////////////////////////////////////
use std::thread;

///メイン関数
fn main() {

    println!("ここは神の世界");
    let kaasuke=String::from("カー助");
    let kaasuke_remain=kaasuke.clone();
    println!("待て待てカー助、お前は大国主の眷属、所有権を移譲しないと人の世に降りられんぞ");
    println!("所有権移譲");
    let handle = thread::spawn( move || {
        println!("恋愛下手なカー子が泣いてる人の世");
        println!("{}参上",kaasuke);
    });

    println!("わたしはカー恵、これから遊べるカラスいないかしら");
    println!("{}参上",kaasuke_remain);

    // 新しいスレッドが完了するのを待つ
    handle.join().unwrap();

}
