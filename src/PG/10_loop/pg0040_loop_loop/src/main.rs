//////////////////////////////
/// 
/// 多重ループ
/// 
/// ///////////////////////////


fn main() {
    let mut dan=1;
    let mut ko=1;

    loop {
        loop {

            print!("{:>3}",dan*ko);
            if ko==9{break;}
            ko+=1;

        }
        
        println!("");
        if dan==9{break;}
        dan+=1;
        ko=1;
    }
}
