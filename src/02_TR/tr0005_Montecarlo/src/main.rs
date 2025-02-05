///////////////////////////
/// 
/// モンテカルロ法で円周率を求める
/// 
/// ///////////////////////
use rand::{Rng,SeedableRng};
//定数
const R:i32=1000;
const TENS:i32=100000;

///メイン関数
fn main() {
    let mut gen=rand::thread_rng();
    let mut x=0;
    let mut y=0;
    let mut in_area_cnt=0;
    let mut judge=false;
    let mut radius=0.0;

   for dummy in 0..TENS
   {
       x=gen.gen_range(0..1000);
       y=gen.gen_range(0..1000);
       judge=is_in_area(x,y);

       if judge
       {
        in_area_cnt=in_area_cnt+1;
       }
   }

   radius=4.0*(in_area_cnt as f64/TENS as f64) ;
   println!("範囲内の点の数{:>4}",in_area_cnt);
   println!("円周率の計算値は{}",radius);
}

///エリア内判定
fn is_in_area(x:i32,y:i32)->bool
{
    let val= x*x+y*y;
    let judge= if R*R>=val{true}else{false};
    return judge;
}
