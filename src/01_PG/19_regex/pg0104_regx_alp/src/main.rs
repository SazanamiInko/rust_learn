/////////////////////////////////
///
/// 正規表現
/// 
//////////////////////////////////
use regex::Regex;

//メイン関数
fn main() {
 
 let alpha="プロジェクトX";
 let no_alpha="プロジェクト*";

 check_alp(alpha);
 check_alp(no_alpha);

}

//アルファベットチェック
fn check_alp(str:&str)
{
    let rule=Regex::new(r"[a-zA-Z]").unwrap();
    if rule.is_match(str)
    {
        println!("{}はアルファベットを含んでます",str);
    }
    else {
        println!("{}はアルファベットを含んでません",str);
    }
}