
////////////////////////////
/// 
/// ハッシュマップ
/// 
///////////////////////////
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() ->std::io::Result<()>{
 
   //お賽銭箱を開けてコインを取り出す。
   let coins=open_osaisen_box()?;
   //コインを整理する
   let coin_box=create_coin_box(coins);
   //コインの枚数を報告する
   report(coin_box);
   Ok(())
}

//お賽銭箱を開ける
fn open_osaisen_box()->std::io::Result<Vec<String> >
{
   
   let mut coins:Vec<String>=Vec::new();
  
   let path=r"C:\Users\Public\Documents\Data\saisenbox.txt".to_string();
   let file=File::open(path)?;
   let reader = BufReader::new(file);

   for line in reader.lines()
   {
      coins.push(line?);
   }

   Ok(coins) 

}

fn create_coin_box(coins:Vec<String>)->HashMap<String,i32>
{
 //ハッシュマップ
 let mut coinbox=HashMap::new();
 for coin in coins
   {
      //取り出したコインが初めてだった。
      coinbox.entry(coin.clone()).or_insert(0);
      //枚数を増やす
      let pick=coinbox.get(&coin).unwrap();
      //枚数を1増やす
      coinbox.insert(coin,pick+1);
  }
  return coinbox;
}

///コインの枚数を報告
fn report(coinbox:HashMap<String,i32>)
{
   for (coin_name,coin_picks) in coinbox
   {
      println!("{}が{}枚入ってます",coin_name,coin_picks);
   }
}
