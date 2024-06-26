//////////////////////////////////////
/// 
/// All検査
/// 
/// ///////////////////////////////////

///メイン関数
fn main() {
    let evens=[32, 72, 356, 444, 1572];
    let odds=[33, 73, 357, 445, 1573];
    let vars=[32, 72, 357, 444, 1572];

    println!("evensは{}",check_even(evens));
    println!("oddsは{}",check_even(odds));
    println!("varsは{}",check_even(vars));
}

///偶数検査
fn check_even(array:[i32;5])->bool
{
   let ret= array.iter().all(|num|num%2==0);
   return ret;
}