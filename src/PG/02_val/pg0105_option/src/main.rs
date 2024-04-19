/////////////////////////////////
///
/// オプション型
/// 
//////////////////////////////////

use std::{collections::HashMap};
use std::fs::File;
use csv;

//メイン関数
fn main() {

  let makura1=String::from("あきつしま");
  let makura2=String::from("しんあきつ");
  let follow=  get_nakura_follow(&makura1);
  let follow2=  get_nakura_follow(&makura2);
 
  report(&makura1, follow);
  report(&makura2, follow2);

}

///枕詞を表示する
fn report(makura:&String,follow:Option<String>)
{
  if follow==None
  {
    println!("{}は枕詞として登録されてません",makura);
  }
  else {
    println!("{}は{}の枕詞です",makura,follow.unwrap());
  }
}

///枕詞を探す
fn get_nakura_follow(makura:&String)->Option<String>
{
   let makuras= read_makura_file();

   let res=makuras.get(makura);
  if res==None
  {
    return  None;
  }
  else {
    let follow=res.unwrap();
      return Some(follow.to_string());
  }
}

///枕詞ファイルを読み込む
fn read_makura_file()->HashMap<String,String>
{
    let mut hash:HashMap<String,String> =HashMap::new();
    let path=r"C:\Users\Public\Documents\Data\Rust\work\pg0105\makura.txt";

    let file=File::open(path).unwrap();
    let mut reader =  csv::ReaderBuilder::new()
                      .has_headers(false)
                      .from_reader(file);

    for result in reader.records()
    {
          let record = result.unwrap();
          hash.insert(record[0].to_string(), record[1].to_string());

    }
    return hash;
}