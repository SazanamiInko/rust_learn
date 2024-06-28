////////////////////////////////
/// 
/// 動物という類型
/// 
/// /////////////////////////////
use crate::logics::animal::animals::{cat,paracket,human};
use crate::logics::animal::traits::Animal;

use super::cat::Cat;


///所持列挙型
pub enum AnimalKind
{
    Cat(cat::Cat),
    Paracket(paracket::Paracket),
    Human(human::Human),
}

///トレイトの実装
impl Animal for AnimalKind
{

    ///使わない前提というか使うことができないコンストラクタ
    fn new(name:&str)->Self {
        
     return AnimalKind::Cat(Cat::new(name));
        
    }

    ///格納した動物を返す
    fn walk(&self) {

        match self {
            AnimalKind::Cat(cat)=>{cat.walk();}
            AnimalKind::Paracket(paracket)=>{paracket.walk();}
            AnimalKind::Human(human)=>{human.walk();}
        }
    }
    fn cry(&self) {
        match self {
            AnimalKind::Cat(cat)=>{cat.cry();}
            AnimalKind::Paracket(paracket)=>{paracket.cry();}
            AnimalKind::Human(human)=>{human.cry();}
        }
    }
}