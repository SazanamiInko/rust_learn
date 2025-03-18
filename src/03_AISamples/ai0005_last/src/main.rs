//////////////////////
/// 
/// 末尾の取得
/// 
/// ///////////////////

///主関数
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    if let Some(last) = v.last() {
        println!("Last element: {}", last);
    } else {
        println!("Vector is empty");
    }
}
