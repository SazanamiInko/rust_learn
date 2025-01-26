/////////////////////////////////////
/// 
/// ぽかぽかてい環境書き込み
/// 
/// /////////////////////////////////
mod logics;
use crate::logics::setting_apis;

///主関数
fn main() {
  setting_apis::write_default();
}
