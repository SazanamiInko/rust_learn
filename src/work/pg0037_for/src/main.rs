////////////////////////////
/// 
/// for文
/// 
////////////////////////////

/// .メイン関数
fn main() {
    let teachers=["佐々木","ライオン丸","きみちゃん","たにし君","まゆずみ"];
    let mut n=1;
    for teacher in teachers.iter()
    {
        println!("{}組:{}先生",n,teacher);
        n+=1;

    } 
 
}
