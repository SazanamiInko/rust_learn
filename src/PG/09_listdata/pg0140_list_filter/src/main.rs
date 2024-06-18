/////////////////////////////////////////////
/// 
/// 文字の除去(Filter)
/// 
/// ////////////////////////////////////////

///メイン関数
fn main() {
  let tel=String::from("888-999-1111");

  let remove_tel:String=tel.chars()
                    .filter(|char_data|*char_data!='-')
                    .collect();

  println!("除去前:{}",tel);
  println!("除去後:{}",remove_tel);
}
