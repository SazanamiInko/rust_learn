///////////////////////////////
/// 
/// 定数構造体
/// 
/// ////////////////////////////
mod tax;
fn main() {
    let tax=tax::tax::create();
    println!("税率は{}です。",tax.get_tax_rate());
}
