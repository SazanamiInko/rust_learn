/////////////////////////////////////
/// 
/// 配列の合計する関数
/// 
/// /////////////////////////////////

///メイン関数
fn main() {
    let array = [1, 2, 3, 4, 5];
    let ans=sum(array);

    println!("{}",ans);
}

///SUM関数
fn sum(array:[i32;5])->i32
{
    let mut _sum=0;

    for i in array
    {
        _sum+=i;
    }
    return  _sum;
}
