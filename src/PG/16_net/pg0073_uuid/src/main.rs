/////////////////////////
///
/// uuid
///
/////////////////////////
use uuid::Uuid;
fn main() {
 
 for _n in 0..10
 {
   println!("{}",Uuid::new_v4().to_string()) ;
 }
}
