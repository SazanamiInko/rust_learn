//////////////////////////////////
/// 
/// 税金の計算
/// 
/// ///////////////////////////////

///税金
pub struct Tax
{
   
}

///実装
impl Tax
{
    ///税率
    const TAX_RATE:f32=0.05;

    ///税率を取得する
    pub fn get_tax_rate(&self)->f32
    {
        return Self::TAX_RATE;
    }
}

///コンストラクタ
pub fn create()->Tax
{
    return Tax{};
}