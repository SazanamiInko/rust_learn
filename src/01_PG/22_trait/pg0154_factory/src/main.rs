//////////////////////////////////////////////////////
/// 
/// Factoryパターン
/// 
/// //////////////////////////////////////////////////
mod logics;

use crate::logics::animal::traits::Animal;
use crate::logics::animal::animals::animal_type::AnimalType;
use crate::logics::animal::animals::animal_kind::AnimalKind;
use std::io;

///メイン関数
fn main() {

    let mut buf:String=String::from("");
    let animal:Box<AnimalKind>;

    //入力をもらう
    println!("いやごきげんよう、のすうこ動物園にようこそ");
    println!("きょうはｓどの動物に会いにきたのかな");
    println!("1:猫,2:インコ");
  
    //入力受付
    _=io::stdin().read_line(&mut buf);
    buf=buf.replace("\r","");
    buf=buf.replace("\n","");

    if buf=="1"
    {
        animal=logics::api::create_animal(AnimalType::Cat, "ままひめ");
    }
    else if  buf=="2" 
    {
        animal=logics::api::create_animal(AnimalType::Paracket, "P");
    }
    else
    {
        animal=logics::api::create_animal(AnimalType::Human, "カー子");   
    }

    animal.walk();
    animal.cry();

}
