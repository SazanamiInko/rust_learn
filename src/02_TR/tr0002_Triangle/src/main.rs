////////////////////////////////
///
///アルキメデスの法則
/// 
//////////////////////////////////

//使用標準ライブラリ
use std::env;


fn main() {

let mut a=0.0;
let mut b =0.0;
let mut ans=0.0;

//入力
let args: Vec<String> = env::args().collect();
let arglen=args.len()-1;
//引数が2個でないときは終了
if(arglen!=2)
{
    println!("引数を2個指定してください。");
    return;
}

//数値に変換できない場合
a=args[1].trim().parse().unwrap();
b=args[2].trim().parse().unwrap();
//処理
ans=getLength(a, b);
//出力
println!("斜辺の長さは{}です。",ans);
}


fn getLength(a:f64,b:f64)->f64
{
    return (a*a+b*b).sqrt();
}
