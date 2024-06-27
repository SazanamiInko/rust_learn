/////////////////////////////////////
/// 
/// メモ帳でファイルを開く
/// 
/// /////////////////////////////////
use std::process::Command;

///メイン関数
fn main() {
    let path=String::from(r"C:\Users\Public\Documents\Data\Rust\work\pg0152\test.txt"); 
    let memo_comnand=String::from("notepad");

   let result=Command::new(memo_comnand)
   .arg(path)
   .status();

   match result
   {
        Ok(status) if status.success() => println!("メモ帳でファイルを開きました"),
        Ok(status) => println!("メモ帳を開くのに失敗しました: {:?}", status),
        Err(e) => println!("コマンド実行に失敗しました: {:?}", e),
   }

}
