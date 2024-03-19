///////////////////////////
/// 
/// 検証
/// 
/// ////////////////////////
use std::io;

//メイン関数
fn main() {

    let mut a:u32=0;
    let mut result:u32=0;
    let mut actual:u32=03;
    let expected:u32=2;
    let mut buf:String =String::new();
   
    println!("鳥取大学の入試問題を解きます");
    println!("a!+2を計算します");
    println!("aを入力してください");
    io::stdin().read_line(&mut buf)
               .expect("入力できませんでした");
    a=buf.trim()
         .parse()
         .expect("数値変換中にエラーが発生しました");

   result=fact(a)+2;
   actual=result%4;   
  
   assert_eq!(actual,expected);

   println!("{}を4で割った余りは2です.",result);

}

//階乗
fn fact(x:u32)->u32
{
    let mut ret=1;

    for i in 1..x+1
    {
        ret=ret*i;
    }

    return ret; 
}
