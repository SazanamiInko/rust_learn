
////////////////////////////////////
/// 
/// 無名関数
/// 
/// /////////////////////////////////

///メイン関数
fn main() {
    let mut coins:Vec<(String,i32,i32)>=Vec::new();
    let mut sum=0;
 
    let make_coin=|name:String,value:i32,picks:i32| { return (name,value,picks)};

     coins.push(make_coin("1円玉".to_string(),1,7));
     coins.push(make_coin("5円玉".to_string(), 5, 2));
     coins.push(make_coin("平等院".to_string(),10,4));

     for coin in coins
     {
         display_coin(coin.clone());
 
         sum+=calc_value(coin.clone());
     }
     println!("合計で{}円持ってます",sum);
 }
 
 //コイン情報表示
 fn display_coin(coin:(String,i32,i32))
 {
     let (name,value,picks)=coin;
     println!("{}を{}枚持ってます",name,picks);
 }
 
 //コイン毎の総額
 fn calc_value(coin:(String,i32,i32))->i32
 {
     let (name,value,picks)=coin;
     value*picks
 }