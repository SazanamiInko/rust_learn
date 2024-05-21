/////////////////////////////
/// 
/// 税金
/// 
/// //////////////////////////
mod tax_calc;

/// 税金
pub struct Tax{

    ///所得税
    pub income:i32,
    //インボイス
    pub invoice:i32
}

impl Tax{

    ///税の総額
    pub fn get_tax()->i32{
        return income+invoice;
    }

    ///控除後の金額
    pub fn get_deduction(&self,source:i32)->i32{
        return source-Self::get_tax();
    }

}

///コンストラクタ
pub fn from(amount:i32)->Tax
{
    return Tax
    {
        income:tax_calc::calc_income(amount),
        invoice:tax_calc::calc_invoice(amount)
    }

}

