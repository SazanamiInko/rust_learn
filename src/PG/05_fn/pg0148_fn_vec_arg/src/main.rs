///////////////////////////////////////////////////
/// 
/// 引数がVecの関数
/// 
/// ///////////////////////////////////////////////

///メイン関数
fn main() {

    let pay_items= vec![1000, 1200, 3100, 1400, 500];
    let source_money=10000;
    let _remain=remain(&source_money, pay_items);

    println!("残りは{}",_remain);
}

///残存を求める
fn remain(source:&i32,items:Vec<i32>)->i32
{
    let mut calc=*source;
    
    for item in items {
        calc-=item;    
    }

    return calc;
}
