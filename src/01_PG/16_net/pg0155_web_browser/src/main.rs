/////////////////////////////////////////////////////
/// 
/// ブラウザ表示
/// 
/// /////////////////////////////////////////////////

use std::io;
use webbrowser;

///メイン関数
fn main() {
const ADULT_AGE:u32=18;
let child_url=String::from("https://www.tokyodisneyresort.jp/tdl/");
let adult_url=String::from("https://www.nta.go.jp/");

 //入力を格納する変数
 let mut buf:String =String::new();

  //入力を促すメッセージ
  println!("年齢を入力してください。");
  //入力受付
  _=io::stdin().read_line(&mut buf);

  let age:u32=buf.replace("\r","")
          .replace("\n","")
          .parse()
          .expect("変換に失敗しました。");
             
   if age<ADULT_AGE
    {
        _=webbrowser::open(&child_url);
    }
    else 
    {
        _=webbrowser::open(&adult_url);
    }
 

}
