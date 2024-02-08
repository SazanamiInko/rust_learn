/////////////////////////////////
///
///関数
/// 
//////////////////////////////////

// . メイン関数

fn main() {
    let mut process=0;

    report_process(process);
    process+=1;
    report_process(process);
    process+=10;
    report_process(process);
    process-=1;
    report_process(process);
    process-=5;
    report_process(process);
}

//報告する関数
fn report_process(percrnt:i32 ){
    println!("現在{:>3}%処理中", percrnt);
}
