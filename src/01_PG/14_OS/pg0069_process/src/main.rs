/////////////////////////
///
/// 別EXEの起動
///
/////////////////////////

use std::{process::Command};

//メイン関数
fn main() {
    exec_process("notepad".to_string());
}

//プロセスを起動する
fn exec_process(process:String)
{
    Command::new(process).spawn()
    .ok()
    .expect("起動できませんでした");
}