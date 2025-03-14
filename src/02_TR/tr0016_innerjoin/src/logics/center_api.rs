/////////////////////////
/// 
/// 本部API
/// 
/// ////////////////////
use std::error::Error;

use crate::logics::components::common::center;
use crate::logics::components::common::path;
use crate::logics::components::datamodel::payment_datmodel::PaymentDataModel;

use super::components::common::enum_list::PaymentType;
use super::components::common::fileio;

///支店リストを返す
pub fn load_branch_list()->Result<Vec<String>,Box<dyn Error>>
{
    //基本のパスを取得
    let base_path=path::get_path();
    //本部のパスを取得
    let center_path=path::get_center_path(&base_path);
    //支店リストもパスを取得
    let branch_list_path=path::get_branch_list_path(&center_path);
    //支店リストの読み込み
    let branch_list_result=center::get_branch_list(&branch_list_path);
    
    match branch_list_result {

        Ok(list)=>{return Ok(list);}
        Err(e)=>{return Err(e);}
    }
}

///結果ファイルを書く
pub fn write_result_file(branch_list:Vec<String>,list:&mut Vec<PaymentDataModel>)->Result<(),Box<dyn Error>>
{
    let path=path::get_path();
    let center_path=path::get_center_path(&path);
    let result_path=path::get_result_path(&center_path);
    let result=center::write_result_file(&result_path, branch_list, list);

    match result
    {
        Ok(_)=>{return Ok(());}
        Err(e)=>{return Err(e);}
    }
}

pub fn write_pay_file(result_list:&Vec<PaymentDataModel>)->Result<(),Box<dyn Error>>
{
    let path=path::get_path();
    let center_path=path::get_center_path(&path);
    let path_500=path::get_pay500_path(&center_path);
    let path_1000=path::get_pay1000_path(&center_path);

    //500円
    let list_500:Vec<String>=result_list.iter()
    .filter(|record|record.pay==PaymentType::Pay500)
    .map(|record|{record.custom_no.clone()})
    .collect();
    
    let result_500=fileio::write_line(&path_500,list_500);

    match result_500
    {
        Ok(())=>{}
        Err(e)=>{return Err(e);}
    }

    //1000円 
    let list_1000=result_list.iter()
    .filter(|record|record.pay==PaymentType::Pay1000)
    .map(|record|{record.custom_no.clone()})
    .collect();
    let result_1000=fileio::write_line(&path_1000, list_1000);

    match result_1000
    {
        Ok(_)=>{return Ok(());}
        Err(e)=>{return Err(e);}       
    }

}