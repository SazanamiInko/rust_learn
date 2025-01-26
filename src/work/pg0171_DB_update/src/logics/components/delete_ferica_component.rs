////////////////////////////////
/// 
/// Ferica削除コンポネント
/// 
///////////////////////////////.
use crate::logics::components::verifys::param_verifys as param_verifys;
use crate::logics::components::verifys::logical_verifys as logical_verifys;
use param_verifys::{ParamVerify,equal_verify::EqualVerify,length_verify::LengthVerify,not_number_verify::NotNumberVerify};
use logical_verifys::{LogicalVerify,auth_verify::AuthVerify,exists_card_verify::ExistsCardVerify};
use crate::logics::components::dao::master_card::MasterCard;
use crate::logics::components::Component;
use crate::logics::components::common::consts;
use mysql::Transaction;
use std::error::Error;

///Ferica削除コンポネント
pub struct DeleteFericaComponent
{
    delete_ferica:String,
    applicant_ferica:String,
    delete_ferica_label:String,
    applicant_ferica_label:String,
    card_len:u32
}

///実装
impl DeleteFericaComponent
{
    ///コンストラクタ
    pub fn new(delete_ferica:&str,applicant_ferica:&str)->Self
    {
        return DeleteFericaComponent
        {
            delete_ferica:delete_ferica.to_string(),
            applicant_ferica:applicant_ferica.to_string(),
            delete_ferica_label:String::from("削除するカード"),
            applicant_ferica_label:String::from("申請者のカード"),
            card_len:16
        };
    }
}

///コンポネント
impl Component for DeleteFericaComponent
{
    ///パラメータチェック
    fn check_param(&self)->Result<(),Box<dyn Error>>
    {
        //同値チェック
         EqualVerify::set("削除するカードと申請者のカードチェック",
                        self.delete_ferica_label.as_str(),
                        self.delete_ferica.as_str(),
                        self.applicant_ferica_label.as_str(),
                        self.applicant_ferica.as_str()
                        )
                        .verify()
                        ?;

        //文字列長チェック
        LengthVerify::set(self.delete_ferica_label.as_str(),
                         self.delete_ferica.as_str(),
                         self.card_len)
                        .verify()
                        ?;     

        LengthVerify::set(self.applicant_ferica_label.as_str(),
                          self.applicant_ferica.as_str(),
                          self.card_len)
                        .verify()
                        ?;                 

        //数字チェック        
        NotNumberVerify::set(self.delete_ferica_label.as_str(),
                            self.delete_ferica.as_str()) 
                            .verify()?;   

        NotNumberVerify::set(self.applicant_ferica_label.as_str(),
                            self.applicant_ferica.as_str()) 
                        .verify()?;   
        
        return Ok(());

    }

    ///相関チェック
    fn check_logical(&self,tran:&mut Transaction)->Result<(),Box<dyn Error>>
    {

        ExistsCardVerify::set(self.delete_ferica_label.as_str(),
                            self.delete_ferica.as_str()) 
                        .verify(tran)?;


       ExistsCardVerify::set(self.applicant_ferica_label.as_str(),
                             self.applicant_ferica.as_str()) 
                        .verify(tran)?;
    
        //カード承認者
        AuthVerify::set(self.applicant_ferica_label.as_str(), 
                        self.applicant_ferica.as_str())
                   .verify(tran)
                   ?;

        return Ok(());
    }

    ///実行
    fn execute(&self,tran:&mut Transaction)->Result<(),Box<dyn Error>>
    {
        //カードを取得
        let mut delete_card=MasterCard::from(self.delete_ferica.as_str(),tran)
                                        .unwrap();
        //削除フラグを立てて更新
        delete_card.deleteflg=consts::DELETEDFLG_ON;
        _=delete_card.update(tran);
        return Ok(());
    }
}