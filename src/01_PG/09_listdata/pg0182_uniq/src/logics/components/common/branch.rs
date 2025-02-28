//////////////////////////////////
/// 
/// 支店
/// 
/// //////////////////////////////
use crate::logics::components::common::fileio;
use std::error::Error;

///支店リストを読み込む
pub fn get_branch_list()->Result<Vec<String>,Box<dyn Error>>
{
    let path=r"C:\Users\Public\Documents\data\Rust\pg0182\branch_list.txt";
    let ret:Vec<String>=fileio::read_line(path)
        .unwrap();
    return Ok(ret);
}

