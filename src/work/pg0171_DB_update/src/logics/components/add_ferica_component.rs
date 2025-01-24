////////////////////////////////
/// 
/// Ferica登録コンポネント
/// 
///////////////////////////////.
use crate::logics::components::verifys::param_verifys as param_verifys;
use crate::logics::components::verifys::logical_verifys as logical_verifys;
use param_verifys::{ParamVerify,equal_verify::EqualVerify,length_verify::LengthVerify,not_number_verify::NotNumberVerify};
use logical_verifys::{LogicalVerify,auth_verify::AuthVerify,not_exists_card_verify::NotExistsCardVerify};
use crate::logics::components::dao::master_card::MasterCard;
use crate::logics::components::Component;
use mysql::Transaction;
use std::error::Error;

///Ferica登録コンポネント
pub struct AddFericaComponent{
    new_ferica:String,
    adder_ferica:String,
    new_ferica_label:String,
    adder_ferica_label:String,
    card_len:u32
}

///実装
impl AddFericaComponent{

    ///コンストラクタ
    pub fn new(new_ferica:&str,adder_ferica:&str)->Self
    {
        return AddFericaComponent
        {
            new_ferica:new_ferica.to_string(),
            adder_ferica:adder_ferica.to_string(),
            new_ferica_label:String::from("登録するカード"),
            adder_ferica_label:String::from("登録者のカード"),
            card_len:16
        }

    }
}

///コンポネント
impl Component for AddFericaComponent
{
    ///パラメータチェック
    fn check_param(&self)->Result<(),Box<dyn Error>>
    {
        //同値チェック
        EqualVerify::set("登録するカードと登録者のチェック",
                         self.new_ferica_label.as_str(), 
                         self.new_ferica.as_str(), 
                         self.adder_ferica_label.as_str(), 
                         self.adder_ferica.as_str())
                        .verify()
                        ?;
        
        //文字列長チェック
        LengthVerify::set(self.new_ferica_label.as_str(),
                          self.new_ferica.as_str(),
                          self.card_len)
                        .verify()
                        ?;     
    
        LengthVerify::set(self.adder_ferica_label.as_str(),
                          self.adder_ferica.as_str(),
                          self.card_len)
                        .verify()
                        ?;                 
        //数字チェック        
        
        NotNumberVerify::set(self.new_ferica_label.as_str(),
                             self.new_ferica.as_str()) 
                         .verify()
                         ?;   
        
        NotNumberVerify::set(self.adder_ferica_label.as_str(),
                             self.adder_ferica.as_str()) 
                             .verify()
                            ?;   

        return Ok(());
    
    }

    ///相関チェック
    fn check_logical(&self,tran:&mut Transaction)->Result<(),Box<dyn Error>>
    {
        //カード存在チェック
        NotExistsCardVerify::set(self.new_ferica_label.as_str(), 
                                 self.new_ferica.as_str())
                            .verify(tran)
                            ?;

        //カード承認者
        AuthVerify::set(self.adder_ferica_label.as_str(), 
                        self.adder_ferica.as_str())
                   .verify(tran)
                   ?;

        return Ok(());
    }

    ///実行
    fn execute(&self,tran:&mut Transaction)->Result<(),Box<dyn Error>>
    {
        let dao=MasterCard::create_new(self.new_ferica.as_str(), self.adder_ferica.as_str());
        _=dao.insert(tran)?;
        return Ok(());
    }

}

