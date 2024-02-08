/////////////////////////////////
///
///参照渡し
/// 
//////////////////////////////////

// . メイン関数
fn main() {
   
    let mut paramA;
    let mut paramB;
    paramA=0;
    paramB=1;
 
    //swap_value(paramA,paramB);
    
    report_value(paramA,paramB);
    swap_ref(&mut paramA,&mut paramB);
    report_value(paramA,paramB);
 }
 
 //関数
 
 /// 値を入れ替えます(噓つき)
 /// *`paramA` - 入れ替える値A
 /// *`paramB` - 入れ替える値B
 //fn swap_value(paramA:i32,paramB:i32)
 //{
 //    let mut buf;
 //    buf=paramA;
 //    paramA=paramB;
 //    paramB=buf;
 //}
 
 /// 値を入れ替えます
 /// *`paramA` - 入れ替える値A
 /// *`paramB` - 入れ替える値B
 fn swap_ref(paramA:&mut i32,paramB:&mut i32)
 {
     let buf;
     
     buf=*paramA;
     *paramA=*paramB;
     *paramB=buf;
 }
 
 /// 報告
 /// *`paramA` - 値A
 /// *`paramB` - 値B
 fn report_value(paramA:i32,paramB:i32)
 {
     println!("");
     println!("paramAは{}",paramA);
     println!("paramBは{}",paramB);
 }