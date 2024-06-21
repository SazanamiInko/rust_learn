/////////////////////////////////////
///
/// 割引マスタDAO
/// 
/// //////////////////////////////////

///割引マスタ
pub struct Discount
{
    ///管理番号
    pub id:i32,
    ///割引コード
    pub code:String,
    ///割引名
    pub name:String,
    ///タイムセールフラグ
    pub is_time:bool,
    ///タイムセール開始時
    pub from_hour:u32,
    ///割引率
    pub rate:f32,
    ///削除フラグ
    pub deleteflg:bool
}


///割引マスタ取得
pub fn get_time_discount()->Result<Vec<Discount>,Box<dyn Error>>
{

}