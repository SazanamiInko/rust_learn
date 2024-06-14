//////////////////////////////////
/// 
/// 文字列の切り取り
/// 
/// //////////////////////////////

const START:usize=8;
const END:usize=13;

///メイン関数
fn main() {
   let barcode=String::from("24061400000012");
   let goods_code=get_goods_code(barcode.as_str());
   
   println!("バーコード:{}",barcode);
   println!("商品コード:{}",goods_code);
}

///商品コードの取得
fn get_goods_code(barcode:&str)->String
{
   let goods_code=barcode[START..END].to_string();
   return goods_code; 
}
