////////////////////////////////
///
///文字→数字変換
/// 
//////////////////////////////////

// . メイン関数
fn main() {
    let str_num_a="2";
    let str_num_b="17";
    
    let mut num_a=0;
    let mut num_b=0;
    let mut ans=0;
    
    num_a=str_num_a.parse().unwrap();
    num_b=str_num_b.parse().unwrap();
    ans=num_a+num_b;
    
    println!("{}+{}={}",str_num_a,str_num_b,ans);
    }
    