/////////////////////////////////////////
/// 
/// 管理FericaカードDAO
/// 
/// //////////////////////////////////////
use mysql::*;
use prelude::Queryable;
use std::error::Error;

///管理Fericaカード
pub struct MasterCard
{
    ///管理番号
    pub id : i32,
    ///FericaID
    pub m_id : String,
    ///登録Ferica
    pub add_m_id : String,
    ///承認Ferica
    pub confirm_m_id: String,
    ///承認権
    pub confirm_auth: bool,
    ///削除フラグ
    pub deleteflg : bool
}

///実装
impl MasterCard

{
    ///コンストラクタ
    pub fn new(id:i32,
               m_id :&str,
               add_m_id :&str,
               confirm_m_id:&str,
               confirm_auth:bool,
               deleteflg:bool)->Self
    {
        return  MasterCard
        {
                id : id,
                m_id :m_id.to_string(),
                add_m_id : add_m_id.to_string(),
                confirm_m_id: confirm_m_id.to_string(),
                confirm_auth:confirm_auth,
                deleteflg : deleteflg
        };

    }

    ///コンストラクタ
    pub fn create_new(m_id :&str,add_m_id :&str)->Self
    {
        return  MasterCard
        {
                id : 0,
                m_id :m_id.to_string(),
                add_m_id : add_m_id.to_string(),
                confirm_m_id: String::from(""),
                confirm_auth:false,
                deleteflg : false
        };

    }

    ///DBからDAO取得
    pub  fn from(ferica_id :&str,tran:&mut Transaction)->Option<Self>
    {
        
     let query=r"SELECT id as id, 
                              mID as m_id,
                              add_mID as add_m_id,
                              confirm_mID as confirm_m_id,
                              confirm_auth as confirm_auth, 
                              deleteflg as deleteflg 
                             FROM m_master_card
                             WHERE deleteflg=0
                            AND mID = :key";

    let mut ret =tran.exec_map(query,
        params!("key"=>ferica_id),
        |(
         id,
         m_id,
         add_m_id,
         confirm_m_id,
         confirm_auth, 
         deleteflg)|
                            
                                 MasterCard{
                                    id :id,
                                    m_id: m_id,
                                    add_m_id:add_m_id,
                                    confirm_m_id:confirm_m_id,
                                    confirm_auth:confirm_auth,
                                    deleteflg:deleteflg}
                                                        
                        ).unwrap();
        if ret.len()==0
        {
            return None;
        }
        let card=ret.remove(0);
        return Some(card);
    }

    ///データ登録
    pub fn insert(&self,tran:&mut Transaction)->Result<u32,Box<dyn Error>>
    {
        let query=r"INSERT INTO m_master_card (mID,add_mID,confirm_mID,confirm_auth,deleteflg) 
                    VALUES (:new_card,:auther_card,null,false,false)";
            tran.exec_drop(query,
            
              params!{
                "new_card"=>self.m_id.clone(),
                "auther_card"=>self.add_m_id.clone()
            })?;
        
        return Ok(1);
    }

///マスターカードの件数を取得
pub fn count(tran:&mut Transaction)->Result<u32,Box<dyn Error>>
{

    let query=r"SELECT COUNT(id) FROM m_master_card ";
    let count=tran.query_first(query)?;

    match count
    {
        None=>
        {
            return Ok(0);
        },
       Some(cun)=>
        {
            return Ok(cun);
        }
    }
}

    ///データ更新
    pub fn update(&self,tran:&mut Transaction)->Result<u32,Box<dyn Error>>
    {
        let query=r"UPDATE m_master_card SET 
        add_mID = :new_add,
        confirm_mID = :new_confirm,
        confirm_auth = :new_confirm_auth,
        deleteflg = :new_deleteflg 
        WHERE mID = :key";

        let res=tran.exec_drop(query,
        params!
        {
            "new_add"=>self.add_m_id.clone(),
            "new_confirm"=>self.confirm_m_id.clone(),
            "new_confirm_auth"=>self.confirm_auth,
            "new_deleteflg"=>self.deleteflg,
            "key"=>self.m_id.clone()
        }
        );

        match  res {
            Ok(_)=>{},
            Err(e)=>
            {
                 return Err(Box::new(e));
            }
        }
        
        return Ok(1);
    }

    ///データ削除
    pub fn delete(tran:&mut Transaction)->Result<u32,Box<dyn Error>>
    {
        //最初の件数
        let before_count=MasterCard::count(tran)
                              .unwrap();
        
        //削除
        let query="DELETE FROM m_master_card  WHERE deleteflg=1";
        let ret=tran.exec_drop(query,());
        match ret {
            Ok(_)=>{},
            Err(e)=>
            {
                 return Err(Box::new(e));
            }
        }

        //最後の件数
        let after_count=MasterCard::count(tran)
                             .unwrap();

        return Ok(before_count-after_count);
    }

}

