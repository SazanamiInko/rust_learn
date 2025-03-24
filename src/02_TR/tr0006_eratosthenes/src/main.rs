use std::sync::TryLockResult;

////////////////////////////////
/// 
/// エラトステネスのふるい
/// 
/// ////////////////////////////

///主関数
fn main() {

    let mut mark: [bool; 100]=[true;100];
    let range=mark.len()/2;

    for  n in 1..range
    {
        if n==1
        {
            continue;
        }
    
        if mark[n]
        {
            let mut m=2*n;
            while m<=mark.len()-1
            {
                mark[m]=false;
                m=m+n;
            }
        }
    }

    for p in 1..mark.len()
    {
        if mark[p]
        {
            println!("{}",p);
        }
    }
}