///////////////////////////////
/// 
/// ZOZO採用ページの表示
/// 
/// ////////////////////////////
mod logics;

///メイン関数
fn main() {
   let url=String::from("ｈｔｔｐｓ：／／ｈｒｍｏｓ．ｃｏ／ｐａｇｅｓ／
ｚｏｚｏ／ｊｏｂｓ／１８０９８４６９７３２４１６８８２３０");

//全角から半角に変換
let url=logics::api::convert_to_han(url.as_str());

_=logics::api::open_brouser(url.as_str());

}
