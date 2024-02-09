/////////////////////////////////
///
///四則演算
/// 
//////////////////////////////////

/// . メイン関数

fn main() {
    let left=50;
    let right=17;
    
    let mut ans=0;
  
    //足し算
    ans=left+right;
    println!("{}+{}={}",left,right,ans);
  
    //引き算
    ans=left-right;
    println!("{}-{}={}",left,right,ans);
  
    //掛け算
    ans=left*right;
    println!("{}*{}={}",left,right,ans);
  
    //割り算
    ans=left/right;
    print!("{}/{}={}",left,right,ans);
  
    //余剰
    ans=left%right;
    print!(" 余り....{}",ans);
  
  
  }
  