//////////////////////////
/// 
/// 猫の構造体
/// 
/// //////////////////////
use crate::logics::traits::Animal;

///データ定義
pub struct Cat
{
    pub name:String
}

impl Cat {

    ///コンストラクタ
    pub fn new(name:&str)->Self
    {
        return Cat{name:name.to_string()};
    }
}

impl Animal for Cat {

 //鳴く
 fn cry(&self)
 {
     println!("ニャーニャー")
 }

 //
 fn introduction(&self) {
     println!("吾輩は猫だにゃー、名前は{}だにゃー",self.name);
 }
    
}