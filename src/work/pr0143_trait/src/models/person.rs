use crate::traits::Greet;

pub struct Person {
    pub name: String,
}

impl Greet for Person {
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}