////////////////////////
/// 
/// タプルを返す関数
/// 
/// ////////////////////


//メイン関数
fn main() {
    let t5=make_coin("5円玉", 5, 2);
     let mut sum=0;
 
     println!("真理子さんは");
    display_coin(t5);
    sum+=calc_value(t5);
     println!("合計で{}円持ってます",sum);
 }
 
 //コイン情報作成
 fn make_coin(name:&str,value:i32,picks:i32)->(&str,i32,i32)
 {
     (name,value,picks)
 }
 
 //コイン情報表示
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
 