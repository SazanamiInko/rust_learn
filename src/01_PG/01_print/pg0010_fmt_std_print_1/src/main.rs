/////////////////////////////////
///
///フォーマット表示1(n進法)
/// 
//////////////////////////////////

/// . メイン関数
fn main() {
    //表示用の変数
    let displayVal7=7;
    let displayVa11=11;
  
    //表示
    println!("2進数表示={:b}",displayVal7);
    println!("8進数表示={:o}",displayVal7);
    println!("10進数表示={}",displayVal7);
    println!("16進数表示={:x}",displayVal7);
  
    println!("-------");
    println!("2進数表示={:b}",displayVa11);
    println!("8進数表示={:o}",displayVa11);
    println!("10進数表示={}",displayVa11);
    println!("16進数表示={:x}",displayVa11);
  
  }
  