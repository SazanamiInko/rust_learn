//////////////////////////
/// 
/// 内部結合
/// 
/// ///////////////////////
mod logics;
use logics::center_api::write_result_file;
use logics::bramch_api;
use logics::components::datamodel::payment_datmodel::PaymentDataModel;
use crate::logics::center_api;

///主関数
fn main() 
{
    //結果
    let mut result_list:Vec<PaymentDataModel>=Vec::new();

    //支店リストを取得する
    let branch_list_result=center_api::load_branch_list();
    match branch_list_result.as_ref()
    {
        Ok(_)=>
        {
        }
        Err(e)=>
        {
            println!("{:?}",e);
            println!("支店リストの取得に失敗しました");
        }
    }

    let branch_list:Vec<String>=branch_list_result.unwrap();
    
    let mut pos:i32=0;
    for branch_name in branch_list.clone()
    {
        let come_result=bramch_api::get_come_list(&branch_name);
        
        match come_result.as_ref() 
        {
            Ok(_)=>{}
            Err(e)=>
            {
                println!("{:?}",e);
                println!("来店記録の取得に失敗しました");
                return;
            }
        }

        let come_list=come_result.unwrap();

        //名簿だけにする
       let _custom_list:Vec<String>= come_list.iter()
       .map(|record|{record.custom_no.clone()})
       .collect();
    
        let mut correct_custom_list:Vec<String>=Vec::new();

        //支払いリストの登録
        _custom_list.iter().for_each(|record|
        {
            //重複してたら除外
            if !correct_custom_list.contains(record)
            {
                correct_custom_list.push(record.clone());
            }
        }
        );

        correct_custom_list.iter_mut().for_each(|custom|
        {
            let find_result=result_list.iter_mut().find(|record|record.custom_no==*custom);
            
            if let Some(find_data)=find_result 
            {
                find_data.visits.push(branch_name.clone());
            }
            else
            {
                let mut pay:PaymentDataModel=PaymentDataModel::new(custom.as_str());
                pay.visits.push(branch_name.clone());

                let mut n:i32=0;
                while n<pos{
                    pay.visits_mark.push(false);
                    
                    n=n+1;
                }
                result_list.push(pay);
            }
        });

        //後処理
        result_list.iter_mut().for_each(|record|
        {
            if let Some(visit_name) = record.visits.last()
            {
                if visit_name.to_string()==branch_name.to_string()
                {
                    record.visits_mark.push(true);
                }
                else 
                {
                    record.visits_mark.push(false);
                }
            }
            else 
            {
                record.visits_mark.push(false);
            }
        });        
        pos=pos+1;
    }
    
    //結果を並び替え
    result_list.sort_by(|record_a,record_b|{record_a.custom_no.cmp(&record_b.custom_no)});
   
    let write_result_file_result= write_result_file(branch_list, &mut result_list);

    match  write_result_file_result{
        Ok(_)=>{}
        Err(e)=>
        {
            println!("{:?}",e);
            println!("結果ファイルの書き込みに失敗しました");
            return;
        }
    }
    
   let result_pay=center_api::write_pay_file(&result_list);
    
    match result_pay {
        Ok(_)=>{}
        Err(e)=>
        {
            println!("{:?}",e);
            println!("商品券ファイルの書き込みに失敗しました");
            return;
        }
    }

    println!("商品券の分類処理が終わりました");
}
