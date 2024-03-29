/////////////////////////////////
///
/// 構造体
/// 
//////////////////////////////////

//使用標準ライブラリ
use std::env;
use std::fs::OpenOptions;
use std::io::Write;

//構造体
//売り上げ
struct Sales{
  enter:u32,
  output:u32,
  amount:u32
}

//メイン関数
fn main() {

  let args: Vec<String> = env::args().collect();


    if args.len()!=4
    {
      println!("引数が不正です");
      return;
    }

    let sales_data=create_data(args);

    if !verify(&sales_data)
    {
      return;
    }
    
    write_uriagw(&sales_data);
  println!("売り上げを登録しました");
}

//売り上げデータ作成
fn create_data(args:Vec<String>)->Sales
{
  
  
  let enter=args[1].trim()
                     .parse()
                     .expect("入庫ICの取得に失敗しました。");
  
  let output=args[2].trim()
                      .parse()
                      .expect("出庫ICの取得に失敗しました。");
  
  let amount=args[3].trim()
                      .parse()
                      .expect("出庫ICの取得に失敗しました。");

    return Sales
    {
      enter:enter,
      output:output,
      amount:amount
    }
}

//妥当性チェック
fn verify(data:&Sales)->bool
{
  //入庫ICと出庫ICのチェック
  if data.enter==data.output
  {
    println!("入庫ICと出庫ICが一緒です");
    return  false;
  }

  if data.enter<1 || data.enter>10
  {
    println!("入庫ICが不正です");
    return  false;
  }

  if data.output<1 || data.output>10
  {
    println!("出庫ICが不正です");
    return  false;
  }

  if data.amount>9999
  {
    println!("売上額が不正です");
    return  false;
  }

  return true;
}

//売上登録
fn write_uriagw(data:&Sales) 
{
  let path=r"C:\Users\Public\Documents\Data\Rust\work\pg0091\uriage.dat".to_string();
 let mut write_data :[u8;8]=[0; 8];
 write_data[0]=(data.enter/10) as u8;
 write_data[1]=(data.enter%10) as u8;
 write_data[2]=(data.output/10)as u8;
 write_data[3]=(data.output%10)as u8;
 write_data[4]=(data.amount/1000)as u8;
 write_data[5]=((data.amount%1000)/100)as u8;
 write_data[6]=((data.amount%100)/10)as u8;
 write_data[7]=(data.amount%10)as u8;
  

  let mut file=OpenOptions::new()
  .append(true)
  .open(path)
  .expect("売り上げファイルを開けませんでした。");
 
  file.write_all(&write_data).expect("売り上げファイルの書き込みに失敗しました");

}