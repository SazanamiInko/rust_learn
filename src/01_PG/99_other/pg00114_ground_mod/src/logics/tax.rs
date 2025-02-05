mod tax_rate;

///インボイスの計算
fn calc_invoice(amount:&u32)->u32
{
   let amount_work=*amount as f32;
   let rate= tax_rate::INVOICE_RATE;
    return (amount_work*rate).floor() as u32;
}

///税率の計算
fn calc_tax(amount:&u32)->u32
{
   let amount_work=*amount as f32;
   let rate= tax_rate::TAX_RATE;
    return (amount_work*rate).floor() as u32;
}

///税の報告
pub fn report_tax(amount:&u32)
{
    let invoice=calc_invoice(amount);
    let tax=calc_tax(amount);
    println!("所得税は{}円、インボイスは{}円",invoice,tax);
    println!("これが日本の貧困の正体です。");

}