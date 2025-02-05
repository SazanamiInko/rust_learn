/////////////////////////////////
///
/// 環境変数の作成
/// 
//////////////////////////////////
use std::env;

///メイン関数
fn main() {
   let env_val="pg0110";

   set_env(env_val, "とらにつばさくん");

   match get_env(env_val)
   {
    Some(val)=>println!("{}={}",env_val,val),
    None=>println!("環境変数がありませんでした"),
   }
}

///環境変数取得
fn get_env(env:&str)->Option<String>
{
    match env::var(env)
    {
        Ok(val)=>return Some(val),
        Err(var_error)=>{
            println!("{:?}", var_error);
            return None;
        }
    }
   
}

///環境変数をセットする
fn set_env(key:&str,value:&str)
{
   env::set_var(key, value);
}