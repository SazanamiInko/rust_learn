
////////////////////////////
/// 
/// 階乗（for文の古典的使い方)
/// 
/// ////////////////////////

//メイン関数
fn main() {
    println!("鳥取大学の入試問題を解きます");
    
    let ans=fact(5)+fact(4)+fact(3);
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
