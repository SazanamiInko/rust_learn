/////////////////////////////////
///
///べき乗(浮動小数点)
/// 
//////////////////////////////////

// . メイン関数

fn main() {
    let length :f32 =5.01;
    let mut ans :f32=0.0;

    ans=length.powf(2.0);

    println!("辺の長さが{}の正方形の面積は{}です。",length,ans);

    ans=length.powf(3.0);
    println!("辺の長さが{}の立方体の体積は{}です。",length,ans);
}
