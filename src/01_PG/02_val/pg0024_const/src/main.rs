/////////////////////////////////
///
///定数
/// 
//////////////////////////////////

//定数
const PI:f32=3.14;

//静的


// . メイン関数

fn main() {
    let length :f32 =5.01;
    let mut ans :f32=0.0;
   
    ans=get_demension(length,length);
    println!("辺の長さが{}の正方形の面積は{1:.2}です。",length,ans);

    ans=get_volume(length, length, length);
    println!("辺の長さが{}の立方体の体積は{1:.2}です。",length,ans);
     
    ans=get_cicle_demension(length);
    println!("半径{}の円の面積は{1:.2}です。",length,ans);

}

//関数

/// 面積を返します
/// * `vertical` - 縦の長さ
/// * `horizontal` - 横の長さ
fn get_demension(vertical:f32,horizontal:f32)->f32
{
    let mut ans:f32;
    ans=horizontal*vertical;
    return ans;
}

/// 体積を返します
/// * `vertical` - 縦の長さ
/// * `horizontal` - 横の長さ
/// * `height` - 高さ
fn get_volume(vertical:f32,horizontal:f32,height:f32)->f32
{
    let mut ans:f32;
    ans=horizontal*vertical*height;
    return ans;
}

// 円の面積を返します
/// * `radius` - 半径
fn get_cicle_demension(radius:f32)->f32
{
    let mut ans:f32;
    ans=radius.powf(2.0)*PI;
    return ans;

}
