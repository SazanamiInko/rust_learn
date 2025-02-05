////////////////////////////
/// 
/// フォルダ作成
/// 
/// ////////////////////////
use std::fs;
use chrono::Local;

fn main() {
    let date=Local::now();
    let fmt="%Y%m%d";
    let now_str=date.format(fmt);
    let path=r"C:\Users\Public\Documents\Data\Rust\work/";
    let dirname=format!("{}{}",path,now_str);
  
   match fs::create_dir(dirname.to_string()) {
    Ok(())=>{println!("{}の作成しました。",dirname);}
    Err(_e)=>{println!("{}の作成に失敗しました。",dirname);}
    }
}
