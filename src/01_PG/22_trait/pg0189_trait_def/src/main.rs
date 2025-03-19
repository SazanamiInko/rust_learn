/////////////////////////////////
/// 
/// トレイとの共通メソッド（デフォルト）
/// 
/// //////////////////////////////
mod logics;

use logics::traits::Animal;
use logics::animals::{cat::Cat, dog::Dog, sparrow::Sparrow};

///主関数
fn main() {
    let mut animals:Vec<Box<dyn Animal>>=Vec::new();
    let cat=Cat::new("みけ"); 
    let dog=Dog::new("ぽち");
    let sparrow=Sparrow::new();

    animals.push(Box::new(cat));
    animals.push(Box::new(dog));
    animals.push(Box::new(sparrow));

    for animal in animals
    {
        animal.introduction();
        animal.walk();
        animal.cry();
        println!("");
    }
}
