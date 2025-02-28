//////////////////////////////
/// 
/// 重複のないデータ
/// 
/// //////////////////////////
mod logics;
use crate::logics::branch_api;
use crate::logics::custom_api;

///主関数
fn main() {
    
    let mut main_list:Vec<String>=Vec::new();

    //支店リストを取得する
    let branches_result=branch_api::get_branch_list();

    match branches_result.as_ref()
    {
        Ok(_)=>{}
        Err(e)=>
        {
            println!("{:?}",e);
            println!("支店リストの取得に失敗しました");
            return;
        }
    }

    let branches=branches_result.unwrap();

    for branch in branches
    {
        println!("{}支店",branch);
        let branch_custom_result=custom_api::read_custom_read(&branch);

        match  branch_custom_result.as_ref(){
            Ok(_)=>{}
            Err(e)=>
            {
                println!("{:?}",e);
                println!("支店の顧客台帳の失敗しました");
                return;
            }
            
        }
        
        let branch_custom=branch_custom_result.unwrap();

        let merge_result=custom_api::merge_custom_list(main_list, 
                                                    branch_custom);
   
        match merge_result {

            Ok(merged)=>{main_list=merged;}
            Err(e)=>
            {
                println!("{:?}",e);
                println!("支店の台帳の併合に失敗しました");
                return;
            }
            
        }
    }

    let result=custom_api::write_main_custom_list(main_list);

    match result {
        Ok(_)=>{println!("本部の顧客台帳を書き込みました");}
        Err(e)=>
        {
            println!("{:?}",e);
            println!("本部の顧客台帳を書き込みに失敗しました");
            return;
        }
    }

}
