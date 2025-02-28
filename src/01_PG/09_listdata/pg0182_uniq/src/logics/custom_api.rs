//////////////////////////////
/// 
/// 顧客管理API
/// 
/// //////////////////////////
use crate::logics::components::common::{path,fileio};
use std::error::Error;

///支店の顧客台帳を読む
pub fn read_custom_read(branch:&str)->Result<Vec<String>,Box<dyn Error>>

{
    let folder_path=path::get_path();
    let branch_path:String=path::get_branch_path(&folder_path, &branch);
    let ret:Vec<String>=fileio::read_line(&branch_path)?;
    
    return Ok(ret);
}

//顧客台帳を合併する
pub fn merge_custom_list(branch1_list:Vec<String>
                        ,bramch2_list:Vec<String>)
                        ->Result<Vec<String>,Box<dyn Error>>
{
    let mut merged_list:Vec<String>;

    //支店Aをコピー
    merged_list=branch1_list.clone();

    //支店２とマージする。
    bramch2_list.iter().for_each(|record|
    {
       if !merged_list.contains(record)
       {
            merged_list.push(record.clone());
       }
    }
    );

    //並び替え
    merged_list.sort_by(|a,b|a.cmp(b));
    
    return Ok(merged_list);
}

//顧客台帳を出力する。
pub fn write_main_custom_list(main_list:Vec<String>)->Result<(),Box<dyn Error>>
{
    let path=path::get_main_path();
    _=fileio::write_line(path.as_str(), main_list)?;
    return Ok(());
}