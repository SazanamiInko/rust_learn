//////////////////////////////////////////////
/// 
/// 3項演算子
/// 
/// //////////////////////////////////////////

//メイン関数
fn main() {
    let a=321;
    let b=56;
    let mut mini=0;
    let mut big=0;
    
    
    mini=if a < b { a } else { b };
    big=if b < a { a } else { b };
    
        println!("{} < {} です",mini,big);
    }
    