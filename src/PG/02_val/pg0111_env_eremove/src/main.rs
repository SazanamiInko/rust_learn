/////////////////////////////////
///
/// 環境変数の削除
/// 
//////////////////////////////////
use std::env;

///メイン関数
fn main() {
   let env_val="pg0111";

   set_env(env_val, "とむにつばさたん");
   report(env_val);
   delete_env(env_val);
   report(env_val);
}

///報告する
fn report(env:&str)
{
    match get_env(env)
    {
     Some(val)=>println!("{}={}",env,val),
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

///環境変数を削除する
fn delete_env(env:&str)
{
    env::remove_var(env);
}