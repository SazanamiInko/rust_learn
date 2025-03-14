//////////////////////////////////
/// 
/// 本部
/// 
/// //////////////////////////////
use crate::logics::components::common::fileio;
use crate::logics::components::datamodel::payment_datmodel::PaymentDataModel;
use std::error::Error;


///支店リストを読み込む
pub fn get_branch_list(path:&str)->Result<Vec<String>,Box<dyn Error>>
{
    println!("{}",path);
     let ret:Vec<String>=fileio::read_line(path)
        .unwrap();
    return Ok(ret);
}

//結果ファイルを書く
pub fn write_result_file(path:&str,branchlist:Vec<String>,list:&mut Vec<PaymentDataModel>)->Result<(),Box<dyn Error>>
{
    //ヘッダーを書く
    let mut header=String::from("顧客番号");
    let mut write_line:Vec<String>=Vec::new();

    for branch in branchlist
    {
        header=header+","+&branch;
    }
    header=header+",商品券";

    write_line.push(header.to_string());

    list.iter_mut().for_each(|record|
        {
            write_line.push(record.get_line());
        }
    );

    let result=fileio::write_line(path, write_line);

    match result {
        Ok(_)=>{return Ok(());}
        Err(e)=>{return Err(e);}
    }
}

///商品券配布リストを書く
pub fn write_payment_file(path:&str,cusom_list:Vec<String>)->Result<(),Box<dyn Error>>
{
    let result=fileio::write_line(path, cusom_list);
    
    match result {
        Ok(_)=>{return Ok(());}
        Err(e)=>{return Err(e);}
    }
}
