////////////////////////////////////////////
/// 
///マスターカードユーテリティ
///
/// ////////////////////////////////////////
use crate::logics::components::dao::master_card as dao_card;
use mysql::*;

///登録済みチェック
pub fn has_master(m_id:&str,tran:&mut Transaction)->bool
{
    let card=dao_card::MasterCard::from(m_id,tran);

    match card{
        None=>{return false;}
        Some(_)=>{return true;}
    }
}

///承認者権有無
pub fn is_auth(m_id:&str,tran:&mut Transaction)->bool
{ 
    let card=dao_card::MasterCard::from(m_id,tran);

    match card{
        None=>{return false;}
        Some(card_data)=>
        {
            if !card_data.confirm_auth
            {
                return false;
            }  
        }
    }
    return true; 
}