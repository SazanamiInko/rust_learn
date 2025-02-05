///////////////////////////////////////////
/// 
/// 文字列の数値チェック
/// 
/// ////////////////////////////////////////
mod logics;


///メイン関数
fn main() {
  let cards=vec!["1234567890123456","123456789X123456"];

    for card in cards.iter() {
        match logics::ferica_api::check(card)
        {
            Ok(_)=>{println!("{}はOK",card);},
            Err(_e)=>{
                println!("{}",_e);    
                println!("{}はNG",card);
            }
        }
    }

}
