//////////////////////////
/// 
/// 支店API
/// 
/// //////////////////////
use std::error::Error;
use crate::logics::components::common::path;
use crate::logics::components::datamodel::come_datamodel::ComeDataModel;
use crate::logics::components::common::branch;
///来店記録の読み取り
pub fn get_come_list(branch_name:&str)->Result<Vec<ComeDataModel>,Box<dyn Error>>
{
    //パスを取得
    let base_path= path::get_path();
    let branch_path=path::get_come_path(&base_path, branch_name);
    let come_result=branch::load_come_file(&branch_path);

    match  come_result{
        Ok(come)=>{return Ok(come);}
        Err(e)=>{return Err(e);}
    }
}