//////////////////////////
/// 
/// 犬の構造体
/// 
/// //////////////////////
use crate::logics::traits::Animal;

///データ定義
pub struct Dog
{
    pub name:String
}

impl Dog {

    ///コンストラクタ
    pub fn new(name:&str)->Self
    {
        return Dog{name:name.to_string()};
    }
}

impl Animal for Dog {

 //鳴く
 fn cry(&self)
 {
     println!("わん")
 }

 //
 fn introduction(&self) {
     println!("僕は犬だわん、名前は{}だわん",self.name);
 }
    
}