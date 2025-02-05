use crate::logics::sallaly;

/////////////////////////////////
///
/// 給与計算
/// 
//////////////////////////////////
mod logics;

#[doc = r"メイン関数"]
fn main() {
  println!("給与計算");

  let sallalys= logics::sallaly::open_jisseki();
  logics::sallaly::write_sallery(sallalys)
                  .expect("給与ファイル作成失敗");

  println!("給与計算しました");
}
