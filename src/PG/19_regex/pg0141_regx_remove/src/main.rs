/////////////////////////////////////////////
/// 
/// 文字の除去(正規表現)
/// 
/// ////////////////////////////////////////
use regex::Regex;

///メイン関数
fn main() {
  let tel=String::from("888-999-1111");
  let remove_rule=Regex::new("-").expect("ルール作成に失敗しました");
  let remove_tel=remove_rule.replace_all(&tel, "")
                                    .to_string();

  println!("除去前:{}",tel);
  println!("除去後:{}",remove_tel);
}
