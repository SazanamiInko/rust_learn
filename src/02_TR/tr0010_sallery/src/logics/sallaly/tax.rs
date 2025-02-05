/////////////////////////////
/// 
/// 税金
/// 
/// //////////////////////////

/// 税金構造体
pub struct Tax{

    ///所得税
    pub income:i32,
    //インボイス
    pub invoice:i32
}

///実装
impl Tax{

    ///控除後の金額
    pub fn get_deduction(&self,source:i32)->i32{
        return source-self.income+self.invoice;
    }

}

///コンストラクタ
pub fn from(amount:&i32)->Tax
{
    return Tax
    {
        income:calc_income(&amount),
        invoice:calc_invoice(&amount)
    }

}

 ///所得税の税率
 const INCOME_RATE:f32=0.05;

 ///インボイスの税率
 const INVOICE_RATE:f32=0.10;

 //所得税の計算
fn calc_income(amount:&i32)->i32
 {
     let work=*amount as f32;
     return (work*INCOME_RATE).floor() as i32;
 }

 ///インボイスの計算
 fn calc_invoice(amount:&i32)->i32
 {
     let work=*amount as f32;
     return (work*INVOICE_RATE).floor() as i32;
 }