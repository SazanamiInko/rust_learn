/////////////////////////////////
///
/// 対数
/// 
//////////////////////////////////

//メイン関数
fn main() {
  let target:f64=2022.0;
  let logistic=target.log(4.0);
  let rounted=(logistic*100.0).round()/100.0;

  println!("4の{:.2}乗で2022にほぼなります。",rounted);

}
