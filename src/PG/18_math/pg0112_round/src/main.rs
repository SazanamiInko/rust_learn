/////////////////////////////////
///
/// 四捨五入
/// 
//////////////////////////////////

///定数
const SEVEN:f32=7.0;

///メイン関数
fn main() {
   let num1=17.0;
   let num2=18.0;
    
  let (before1,after1)=divide_seven(num1);
  let (before2,after2)=divide_seven(num2);

  println!("{}/{}={}→{}\r\n",num1,SEVEN,before1,after1);
  println!("{}/{}={}→{}\r\n",num2,SEVEN,before2,after2);

}

///７で割った数と四捨五入をする関数
fn divide_seven(num:f32)->(f32,i32)
{
    let before=num/SEVEN;
    let after=before.round();

    return (before,after as i32);
}

