///////////////////////////////////////
/// 
/// 演算子のオーバーロード
/// 
/// ///////////////////////////////////
mod logics;

use crate::logics::circle_seven::CircleSeven;

///主関数
fn main() {
    
    let two=CircleSeven::from(2);
    let three=CircleSeven::from(3);
    let seven=CircleSeven::from(7);
    let eight=CircleSeven::from(8);

    println!("7の環");
    println!("2→{}",two.get());
    println!("3→{}",three.get());
    println!("7→{}",seven.get());
    println!("8→{}",eight.get());
   
    
    let calc1=two+three;
    let calc2=seven+two;
    
    println!("7の環の加算");
    println!("2+3={}",calc1.get());
    println!("7+2={}",calc2.get());
}
