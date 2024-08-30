////////////////////////////////////////////
/// 
/// 慶応志木入試問題(2000年)
/// 
/// ///////////////////////////////////////

///引数を超えない7の倍数を返します
pub fn floor_seven(param:u32)->u32
{
    //定数
    const F_SEVEN:f32=7.0;
    const U_SEVEN:u32=7;


    let mut work_area1=param as f32;

    work_area1=work_area1/F_SEVEN;

    work_area1=work_area1.floor();

    let mut work_area2=work_area1 as u32;

    work_area2=work_area2*U_SEVEN;

    return work_area2;
               
}

///小数点一桁以下を切り捨てる四捨五入
pub fn round_p1(param:f32)->String
{
    //定数
    const KETA_SHIFT:f32=10.0;
    const FLOOR_PARAM:i32=10;
    const KETA:i32=1;

    let mut work_area=KETA_SHIFT*param;

    work_area=work_area+0.5;
  
    work_area=work_area.floor();
   
    let rounded=work_area as i32;

    let point_left=rounded/FLOOR_PARAM;

    let point_right=rounded%FLOOR_PARAM;

    let ret=format!("{}.{}",point_left,point_right);
    return ret;

}

///小数点3桁以下を切り捨てる四捨五入
pub fn round_p3(param:f32)->String
{
    //定数
    const KETA_SHIFT:f32=1000.0;
    const KETA:i32=3;
    const FLOOR_PARAM:i32=1000;

    let mut work_area=KETA_SHIFT*param;

    work_area=work_area+0.5;
  
    work_area=work_area.floor();

    let rounded=work_area as i32;

    let point_left=rounded/FLOOR_PARAM;

    let point_right=FLOOR_PARAM+rounded%FLOOR_PARAM;
    let point_right:String=point_right.to_string()
                                       .chars()
                                       .skip(1)
                                       .collect();

    let ret=format!("{}.{}",point_left,point_right);

    return ret;

}
