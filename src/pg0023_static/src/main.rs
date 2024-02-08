/////////////////////////////////
///
///静的変数(C#ののりでやったら、難しかった。)
/// 
//////////////////////////////////


//標準ライブラリ
use std::sync::atomic::{self, AtomicI32};

//静的変数

/*
進捗
*/
static PROCESS: AtomicI32 = AtomicI32::new(0);


// . メイン関数
fn main() {
   
    report_process();
    count_up(1);
    report_process();
    count_up(10);
    report_process();
    count_up(-1);
    report_process();
    count_up(-5);
    report_process();
}

// . 進捗報告関数
fn report_process(){
    println!("現在{:>3}%処理中", get_process());
}


/// .進捗計上
/// * `degree` - 度合
fn count_up(degree:i32) 
{ 
    PROCESS.fetch_add(degree, atomic::Ordering::SeqCst); 
}


/// .進捗取得
fn get_process() -> i32 
{ 
    return PROCESS.load( atomic::Ordering::SeqCst); 
}


