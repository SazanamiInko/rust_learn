mod models;
mod traits;

use models::person::Person;
use traits::Greet;

fn main() {
    let person = Person { name: String::from("Alice") };
    println!("{}", person.greet());
}