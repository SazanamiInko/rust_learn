/////////////////////////////////
///
///べき乗
/// 
//////////////////////////////////

// . メイン関数
fn main() {
    let length :u32 =5;
    let mut ans :u32=0;

    ans=length.Pow(2);

    println!("辺の長さが{}の正方形の面積は{}です。",length,ans);

    ans=length.Pow(3);
    println!("辺の長さが{}の立方体の退席は{}です。",length,ans);
}
