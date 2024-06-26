/////////////////////////////////////////
/// 
/// 管理FericaカードDAO
/// 
/// //////////////////////////////////////

///管理Fericaカード
pub struct master_card
{
    ///管理番号
    pub id : i32,
    ///FericaID
    pub mID : String,
    ///登録Ferica
    pub add_mID : String,
    ///承認Ferica
    pub confurm_mID: String,
    ///承認権
    pub confirm_auth: bool,
    ///削除フラグ
    pub deleteflg : bool
}

///実装
impl master_card
{
    ///コンストラクタ
    pub fn new(id:&i32,
               mID :&str,
               add_mID :&str,
               confurm_mID:&str,
               confirm_auth:bool,
               deleteflg:&bool)->Self
    {
        return master_card{
                id : id,
                mID :mID.clone() ,
                add_mID : add_mID.clone(),
                confurm_mID: confurm_mID.clone(),
                confirm_auth:confirm_auth,
                deleteflg : deleteflg
        };

    }

    ///Ferica登録
    pub fn insert(&self)->Recult<u32,Box<dyn Error>>
    {}
}

///コンストラクタ
pub fn create_bew_card(mid:&str,add_mID:&str)
{
    return master_card{
        id : 0,
        mID :mID.clone() ,
        add_mID : add_mID.clone(),
        confurm_mID: String::from(""),
        confirm_auth:false,
        deleteflg :false
};
}

///検索
pub fn get(mid:&str)->master_card
{
    
}
