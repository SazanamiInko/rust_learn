/////////////////////////////////////////
/// 
/// 管理FericaカードDAO
/// 
/// //////////////////////////////////////
use mysql::{prelude::Queryable, Transaction};

///管理Fericaカード
pub struct master_card
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
impl master_card
{
    ///コンストラクタ
    pub fn new(id:i32,
               m_id :&str,
               add_m_id :&str,
               confurm_m_id:&str,
               confirm_auth:bool,
               deleteflg:bool)->Self
    {
        return master_card{
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
        
     let query="SELECT id, name FROM m_goods";

    return conn.query_map(query,|(id,name)|
                            {
                                goods::create(id,name)
                            }
                        ).expect("クエリの実行に失敗しました");
                        

    }

    ///Ferica登録
    pub fn insert(&self)->Recult<u32,Box<dyn Error>>
    {}
}

