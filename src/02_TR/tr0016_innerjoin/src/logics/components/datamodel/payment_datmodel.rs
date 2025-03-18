/////////////////////////
/// 
/// 支払い資料
/// 
/// ////////////////////
use crate::logics::components::common::enum_list::PaymentType;

///データ定義
pub struct PaymentDataModel
{
    //顧客番号
    pub custom_no:String,
    //来店リスト
    pub visits:Vec<String>,
    //来店
    pub visits_mark:Vec<bool>,
    //商品券
    pub pay:PaymentType
}

//実装
impl PaymentDataModel
{
    ///コンストラクタ
    pub fn new(custom_no:&str)->Self
    {
        return PaymentDataModel
        {
            custom_no:custom_no.to_string(),
            visits:Vec::new(),
            visits_mark:Vec::new(),
            pay:PaymentType::Pay500
        }
    }

    ///決定
    pub fn decide(&mut self)
    {
        if self.visits.len()==self.visits_mark.len()
        {
            self.pay=PaymentType::Pay1000;
        }
    }

    ///書き出す文字列の取得
    pub fn get_line(&mut self)->String
    {
        let mut line=self.custom_no.clone();
        self.decide();
        for mark in self.visits_mark.clone()
        {
            if mark
            {
                line=line+",〇";
            }
            else 
            {
                line=line+",×";
            }
        }

        if self.pay==PaymentType::Pay500
        {
            line=line+",500";
        }
        else 
        {
            line=line+",1000";
        }
        return line;
    }
} 