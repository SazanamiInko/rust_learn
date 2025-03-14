//////////////////////////////////
/// 
/// 来店記録
/// 
/// ///////////////////////////////

///データ定義
pub struct ComeDataModel
{
    //顧客番号
    pub custom_no:String,
    //来店日時
    pub come:String,
}

///実装
impl ComeDataModel
{
    ///コンストラクタ
    pub fn new(custom_no:&str,
               come:&str)->Self
    {
        return ComeDataModel
        {
            custom_no:custom_no.to_string(),
            come:come.to_string(),
        }
    }
}