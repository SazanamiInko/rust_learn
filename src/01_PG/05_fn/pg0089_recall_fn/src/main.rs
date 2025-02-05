
////////////////////////////
/// 
/// 階乗（再帰呼び出し)
/// 
/// ////////////////////////

//メイン関数
fn main() {
    println!("鳥取大学の入試問題を解きます");
    
    let ans=fact_recall(5)+fact_recall(4)+fact_recall(3);
    println!("5!+4!+3!={}",ans);
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

//階乗(再帰呼び出し)
fn fact_recall(x:u32)->u32
{
    if x==1
    {
        return x;
    }

    return x*fact_recall(x-1);
}
