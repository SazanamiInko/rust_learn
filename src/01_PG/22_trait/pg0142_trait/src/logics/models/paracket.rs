/////////////////////////////////
/// 
/// インコ構造体
/// 
/// /////////////////////////////
use crate::logics::animal::Animal;

///インコ構造体
pub struct Paracket{
    ///名前
    pub name:String,
}

///実装
impl Animal for Paracket
{
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

///コンストラクタ
pub fn create(name:&str)->Paracket
{
    return Paracket{name:name.to_string()};
}