/////////////////////////////////
///
///BMIを計算
/// 
//////////////////////////////////

//利用標準ライブラリ
use std::io;

//定数
const  TOMETER:f32=100.0;
const ZIZYOU:f32=2.0;

///メイン関数
fn main() {
  
  //変数

  //体重
  let mut _weight:f32=0.0;
  //身長
  let mut _height:f32=0.0; 

  //BMI
  let mut _bmi_result:f32=0.0;
  //適正体重
  let mut _best_weight_result:f32=0.0;

  //入力
  //体重
  read_std_in("体重",&mut _weight);
  //身長
  read_std_in("身長",&mut _height);


  //処理
  //BMI値
  _bmi_result=get_Bmi(_weight, _height);
  //適正体重
  _best_weight_result=get_Best_Weight(_height);

  //出力
  println!("あなたのBMI価は{:.2}です",_bmi_result);
  println!("適正体重は{:.270.5}です",_best_weight_result);


}

//関数

/// BMI値を返します
/// *weight` - 体重
/// *height` - 身長
fn get_Bmi(weight:f32,height:f32)->f32
{
    let mut result:f32=0.0;
    result=weight/(height/TOMETER).powf(ZIZYOU);
    return  result;
}

/// 適正体重を返します
/// *height` - 身長
fn get_Best_Weight(height:f32)->f32
{
    const KEISU:f32=22.0;
    let mut result:f32=0.0;

    result= (height/TOMETER).powf(ZIZYOU)*KEISU;

    return  result;
}

// 標準入力を行います
/// *message` - メッセージ
/// *in_param` - パラメータ
fn read_std_in(message:&str,in_param:&mut f32)
{
    let mut buff:String=String::new();
    let mut work:&str="";
    println!("{}を入力してください",message);

    io::stdin().read_line(&mut buff).expect("入力でエラーが発生しました");
    work=buff.trim();
    *in_param= work.parse().unwrap();
   
    println!("");
}



