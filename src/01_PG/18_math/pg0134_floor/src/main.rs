/////////////////////////////////
/// 
/// フロア
/// 
/// ////////////////////////////

///メイン関数
fn main() {
    let w=get_by_over_5(9);
    println!("9より小さい5の倍数は{}です",w);
}

///引数以上の5の倍数を返す
fn get_by_over_5(i:i32)->i32
{
    let work=i as f32;
    const FIVE:f32=5.0;

   return  ((work/FIVE).floor()*FIVE) as i32;

}
