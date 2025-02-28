////////////////////////
/// 
/// AIサンプル:Firstordefault
/// 
//////////////////////////

///FirstOrDefault
fn first_or_default<T: Clone>(iter: impl IntoIterator<Item = T>, default: T) -> T 
{
    iter.into_iter().next().unwrap_or(default)
}

///主関数
fn main() {
    let items: Vec<i32> = vec![];
    let result = first_or_default(items, 10);
    println!("{}", result); // 10（デフォルト値）
}
