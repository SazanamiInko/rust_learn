/////////////////////////////////////////
/// 
/// 管理FericaカードDAO
/// 
/// //////////////////////////////////////
use mysql::{prelude::Queryable, Transaction};
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
    pub confurm_m_id: String,
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
               confurm_m_id:&str,
               confirm_auth:bool,
               deleteflg:bool)->Self
    {
        return  MasterCard
        {
                id : id,
                m_id :m_id.to_string(),
                add_m_id : add_m_id.to_string(),
                confurm_m_id: confurm_m_id.to_string(),
                confirm_auth:confirm_auth,
                deleteflg : deleteflg
        };

    }

    //DBからDAO取得
    pub  fn from(m_id :&str,tran:&mut Transaction)->Option<Self>
    {
        
     let query=r"SELECT id as id, 
                              mID as m_id,
                              add_mID as add_m_id,
                              confurm_mID as confurm_m_id,
                              confirm_auth as confirm_auth, 
                              deleteflg as deleteflg 
                             FROM m_master_card
                             WHERE deleteflg=0 ";

    let ret =tran.query_map(query,|(
         id,
         m_id,
         add_m_id,
         confurm_m_id,
         confirm_auth, 
         deleteflg)|
                            
                                 MasterCard{
                                    id :id,
                                    m_id: m_id,
                                    add_m_id:add_m_id,
                                    confurm_m_id:confurm_m_id,
                                    confirm_auth:confirm_auth,
                                    deleteflg:deleteflg}
                                                        
                        ).unwrap();
        if ret.len()==0
        {
            return None;
        }

        return Some(ret[0]);
    }

    ///Ferica登録
    //pub fn insert(&self)->Result<u32,Box<dyn Error>>
    //{}
}

