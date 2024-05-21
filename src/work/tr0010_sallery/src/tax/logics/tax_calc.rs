/////////////////////////////
/// 
/// 税金
/// 
/// //////////////////////////

    ///所得税の税率
    const INCOME_RATE:f32=0.05;

    ///インボイスの税率
    const INVOICE_RATE:f32=0.10;

    //所得税の計算
    pub fn calc_income(&self,amount:&i32)->i32
    {
        let work=amount as f32;
        return (amount*INCOME_RATE).floor;
    }

    ///インボイスの計算
    pub fn calc_invoice(amount:&i32)->i32
    {
        let work=amount as f32;
        return (amount*INVOICE_RATE).floor;
    }
