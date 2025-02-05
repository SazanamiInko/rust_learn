/////////////////////////////////
///
/// Result型
/// 
//////////////////////////////////
use regex::Regex;
use std::io;

///メイン関数
fn main() {

    let mut work= String::from("");
    println!("しりとりです。文字を入力してください。");
    io::stdin().read_line(&mut work).expect("入力でエラーが発生しました");

    let res=judge_shiritori(&work);

    if let Ok(judge)=res
    {
        if judge
        {
            println!("しりとりOK")
        }
        else 
        {
            println!("しりとり負け")
        }
    }

    if let Err(message)=res
    {
        println!("{}",message);
    }


}

///しりとりの判定関数
/// 
/// 引数
/// 
/// word:検査する語
/// 
/// 戻り値
/// 
///しりとり可能ならtrue,
/// 
/// 空文字、アルファベットはルール違反
fn judge_shiritori(word:&str)->Result<bool,String>
{
    //改行文字を取り除く
    let target=word.to_string()
                            .replace("\r\n", "");
    //空文字NG
    if target.is_empty()
    {
       return  Err::<bool, String>(String::from("空文字はルール違反です。"));
    }
    

    //アルファベットはNG
    let rule=Regex::new(r"[a-zA-Z]").unwrap();
    if rule.is_match(&target)
    {
       return Err::<bool, String>(String::from("アルファベットはルール違反です。"));
    }

    if target.len()==3
    {
       return  Err::<bool, String>(String::from("1文字はルール違反です。"));
    }
    //【ん】の判定
    if target.ends_with("ん")
    {
       return Ok::<bool, String>(false);

    }

    return Ok::<bool, String>(true);
}