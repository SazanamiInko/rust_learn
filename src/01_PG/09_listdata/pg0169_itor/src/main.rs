///////////////////////////////////
/// 
/// 反復子
/// 
/// ///////////////////////////////

fn main() {

    //可変配列を用意する
    let mut veci:Vec<u32>=Vec::new();
    //100まで数値を入れる
    for n in 1..100
    {
        veci.push(n);
    }

    let odds:Vec<_>=veci.iter()
                 .filter(|number|*number%2==1)
                 .map(|number|number)
                 .collect();
    
    for i in odds
    {
        print!("{},",i);
    }


}
