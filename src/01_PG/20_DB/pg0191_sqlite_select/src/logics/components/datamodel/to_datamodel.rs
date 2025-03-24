///////////////////////////////
/// 
/// 行先データモデル
/// 
///////////////////////////////

///データ定義
pub struct ToDataModel
{
    //ID
    pub id:i32,
    //着駅
    pub take_off:i32,
    //時間
    pub time:String
}

///実装
impl ToDataModel
{
    ///コンストラクタ
    pub fn new(id:i32,take_off:i32,time:&str)->Self
    {
        return ToDataModel
        {
            id:id,
            take_off:take_off,
            time:time.to_string()
        };
    }
}