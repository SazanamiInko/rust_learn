/////////////////////////
/// 
/// 文字列の結合2
/// 
/// /////////////////////

///メイン関数
fn main() {
    let makura=String::from("時間よ止まれ!!");
    let stand_user=String::from("Dio");
    let stand_name=String::from("ザ・ワールド");

    let serihu_array=[makura.as_str(),
                        stand_user.as_str(),
                        stand_name.as_str()];
                        

    println!("{}",serihu_array.concat());


}
