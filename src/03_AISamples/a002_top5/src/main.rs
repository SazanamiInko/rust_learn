/////////////////////////////
/// 
/// AIサンプル　降順TOP５
/// 
/// //////////////////////////

///主関数
fn main() {
    let mut numbers = vec![10, 5, 8, 3, 15, 7, 12, 20, 1];

    // 降順ソート
    numbers.sort_by(|a, b| b.cmp(a));

    // 上位5つを取得
    let top5: Vec<_> = numbers.iter().take(5).collect();

    println!("{:?}", top5);
}