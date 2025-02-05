use super::animal::animals::animal_kind;
use super::animal::animals::human::Human;
////////////////////////////////////
/// 
/// API
/// 
/// /////////////////////////////////
use super::animal::traits::Animal;
use super::animal::animals::animal_type::AnimalType;
use super::animal::animals::cat::Cat;
use super::animal::animals::paracket::Paracket;
use super::animal::animals::animal_kind::AnimalKind;
///動物を作る
pub fn create_animal(animal:AnimalType,name:&str)-> Box<AnimalKind>
{

    match animal {

        AnimalType::Cat=>
        {
            let _animal=animal_kind::AnimalKind::Cat(Cat::new(name));
            return Box::new(_animal);
        },
        
        AnimalType::Paracket=>
        {
            let _animal=animal_kind::AnimalKind::Paracket(Paracket::new(name));
            return Box::new(_animal); 
        }
        AnimalType::Human=>
        {
            let  _animal=animal_kind::AnimalKind::Human(Human::new(name));
            return Box::new(_animal); 
        }                
    }


}