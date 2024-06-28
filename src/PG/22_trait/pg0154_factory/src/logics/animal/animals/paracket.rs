/////////////////////////////////
/// 
/// インコ構造体
/// 
/// /////////////////////////////
use crate::logics::animal::traits::Animal;

///インコ構造体
pub struct Paracket{
    ///名前
    pub name:String,
}

///Animalトレイトの実装
impl Animal for Paracket
{
    ///コンストラクタ
    fn new(name:&str)->Self
    {
        return Paracket{
            name:name.to_string()
        };
    }

    ///歩く
    fn walk(&self)
    {
        println!("とことこ");
    }

    //鳴く
    fn cry(&self)
    {
        println!("{}ちゃん",self.name);
    }
}
