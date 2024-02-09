////////////////////////
/// 
/// タプルのベクター
/// 
/////////////////////////

///メイン関数
fn main() {
    let mut coins:Vec<(&str,i32,i32)>=Vec::new();
    let mut sum=0;
 
     coins.push(make_coin("1円玉",1,7));
     coins.push(make_coin("5円玉", 5, 2));
     coins.push(make_coin("平等院",10,4));
 
     for coin in coins
     {
         display_coin(coin);
 
         sum+=calc_value(coin);
     }
     println!("合計で{}円持ってます",sum);
 }
 
 //コイン情報を作成
 fn make_coin(name:&str,value:i32,picks:i32)->(&str,i32,i32)
 {
     (name,value,picks)
 }
 
 //コイン情報の表示
 fn display_coin(coin:(&str,i32,i32))
 {
     let (name,value,picks)=coin;
     println!("{}を{}枚持ってます",name,picks);
 }
 
 //コイン毎の総額
 fn calc_value(coin:(&str,i32,i32))->i32
 {
     let (name,value,picks)=coin;
     value*picks
 }
 