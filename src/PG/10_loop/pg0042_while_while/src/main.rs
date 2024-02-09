//////////////////////////
/// 
/// 多重while
/// 
/// ///////////////////////

fn main() {
    let mut dan=1;
    let mut ko=1;
    
    while dan<10 {
        
        while ko<10 {
            
            print!("{:>3}",dan*ko);
            ko+=1;
        }
        println!("");
        dan+=1;
        ko=1;
    }
    
    }
    