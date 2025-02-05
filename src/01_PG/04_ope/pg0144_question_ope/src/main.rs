//////////////////////////////////////////////////
/// 
/// ?演算
/// 
/// //////////////////////////////////////////////
use std::error::Error;

///メイン関数
fn main() {
    match method_a()
    {
        Ok(_)=>{println!("正常終了しました");},
        Err(e)=>{println!("{}",e);}
    }
}

///A関数
fn method_a()->Result<(),Box<dyn Error>>
{
    method_b()?;
    Ok(())
}

///B関数
fn method_b()->Result<(),String>
{
   Err::<(), String>(String::from("まだ開発中なのでよべません"))
}