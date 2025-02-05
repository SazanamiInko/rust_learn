////////////////////////////////
/// 
/// DB接続(取得)
/// 
/// /////////////////////////////
mod logics;

///メイン関数
fn main() {
   
   let mgoods=logics::uriage::get_goods();

   for goods in mgoods
   {
        goods.display();
   }

}
