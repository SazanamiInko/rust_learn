///////////////////////////////
/// 
/// スライス
/// 
/// ///////////////////////////

fn main() {
    let waka=["秋の田の","かりほの庵の","苫あらみ","わが衣手は","露に濡れつつ"];
    let kami=&waka[0..3];
    let simo=&waka[3..5];
    
    println!("上の句");
    
    for ku in kami
    {
        println!("{}",ku);
    }
    
    println!("");
    
    println!("下の句");
    for ku in simo
    {
        println!("{}",ku);
    }   
    }
    