/////////////////////////////////
///
/// 環境変数
/// 
//////////////////////////////////
use std::env::{self, VarError};

///メイン関数
fn main() {
   
   match get_env("USERPROFILE")
   {
    Some(val)=>println!("{}",val),
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