/////////////////////////////////
/// 
/// トレイとの共通メソッド（デフォルト）
/// 
/// //////////////////////////////
mod logics;

use logics::traits::{Animal, Bird};
use logics::animals::{cat::Cat, dog::Dog};
use logics::animals::birds::sparrow::Sparrow;

///主関数
fn main() {
    let mut animals:Vec<Box<dyn Animal>>=Vec::new();
    let cat=Cat::new("みけ"); 
    let dog=Dog::new("ぽち");
    let sparrow=Sparrow::new();

    animals.push(Box::new(cat));
    animals.push(Box::new(dog));
    animals.push(Box::new(sparrow.clone()));

    for animal in animals
    {
        animal.introduction();
        animal.walk();
        animal.cry();
        println!("");
    }
    println!("雀が飛んだ。");
    sparrow.fly();
}
