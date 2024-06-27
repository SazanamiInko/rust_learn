/////////////////////////////////////
/// 
/// メモ帳でファイルを開く(非同期)
/// 
/// /////////////////////////////////
use std::process::Command;

///メイン関数
fn main() {
    let path=String::from(r"C:\Users\Public\Documents\Data\Rust\work\pg0152\test.txt"); 
    let memo_comnand=String::from("notepad");

   let result=Command::new(memo_comnand)
   .arg(path)
   .spawn();

   match result
   {
        Ok(_) => println!("メモ帳でファイルを開きました"),
        Err(e) => println!("コマンド実行に失敗しました: {:?}", e),
   }

}
