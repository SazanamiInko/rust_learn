////////////////////////
/// 
/// ベクター
/// 
/// ////////////////////

//メイン関数
fn main()
{
 let mut leaps:Vec<i32> =Vec::new();
 let mut count =1;
 for year in (1900..2024).step_by(2) 
 {
    if is_leap(year)
    {
      leaps.push(year);
    }
 }

 println!("");
 print!("うるう年は{}個あります。",leaps.len());
 println!("");
 for leap in leaps
 {
    if count%10==0
    {
      println!("{}",leap);
    }
    else 
    {
      print!("{},",leap);   
    }

    count=count+1;
 }


 

}

///うるう年判定
fn is_leap(year:i32)->bool
{
  let mut ret=false;
  let mut mods=year%100;

  if mods==0
  {
    mods=year%400;
    
    if mods!=0
    {
      ret=true;
    }
  }
  else
  {
      mods=year%4;
      if mods==0
      {
        ret=true;
      }
    {
  }
}

  ret

}