//////////////////////////////
/// 
/// ユークリッドの互除法
/// 
/// //////////////////////////

//利用標準ライブラリ
use std::io;

///メイン関数
fn main() {
//宣言
let mut inputA :u32;
let mut inputB:u32;

let mut left:u32;
let mut right:u32;
let mut maxDiv:u32;

//入力
println!("整数を入力してください。");
inputA=pronmpt();
inputB=pronmpt();

//処理

left= if inputA > inputB { inputA } else { inputB };
right=if inputB > inputA { inputA } else { inputB };
maxDiv=calcDiv(left, right);

//出力
println!("{}と{}の最大公約数は{}です。",inputA,inputB,maxDiv);

}

// 入力促進
fn pronmpt()->u32
{
    let mut buff:String=String::new();
    let mut work:&str="";
    let mut input;
   
    io::stdin().read_line(&mut buff).expect("入力でエラーが発生しました");
    work=buff.trim();
    input= work.parse().unwrap();
    return input;
}

//ユークリッドの互除法
fn calcDiv(left:u32,right:u32)->u32
{
    let mut workLeft=left;
    let mut workRight=right;
    let mut mod_num:u32=0; 
   
   loop {
        mod_num=workLeft%workRight;    
        workLeft=workRight;
        workRight=mod_num;
       
       if(mod_num==0){
            break;
        }
    }

    return  workLeft;    
}

