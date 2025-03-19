////////////////////////////
/// 
/// 雀の構造体
/// 
/// ////////////////////////

use crate::logics::traits::Animal;

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

impl Animal for Sparrow {

 //鳴く
 fn cry(&self)
 {
     println!("ちゅんちゅん")
 }

 //
 fn introduction(&self) {
     println!("雀です");
 }

 fn walk(&self) {
     println!("ちょんちょん");
 }
    
}