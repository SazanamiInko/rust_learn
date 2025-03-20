////////////////////////////
/// 
/// 雀の構造体
/// 
/// ////////////////////////
use crate::logics::traits::{Bird, Animal};

#[derive(Clone)]
///データ定義
pub struct Sparrow
{
}

impl Sparrow {

    ///コンストラクタ
    pub fn new()->Self
    {
        return Sparrow{};
    }
}

//動物トレイト
impl Animal for Sparrow {
 //鳴く
 fn cry(&self)
 {
     println!("ちゅんちゅん")
 }

 //自己紹介
 fn introduction(&self) {
     println!("雀です");
 }

 ///歩く
 fn walk(&self) {
     println!("ちょんちょん");
 }    
}


impl Bird for Sparrow {

 fn fly(&self) {
    println!("ぱたぱた");
}
    
}