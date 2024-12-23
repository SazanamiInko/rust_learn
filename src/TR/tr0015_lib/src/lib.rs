mod logics;
use logics::apis;

////////////////////////////////////////
/// 
/// 引数を超えない7の倍数を返す
/// 
/// ////////////////////////////////////
pub fn floor_seven(param:u32)->u32
{
    apis::floor_seven(param)
}

////////////////////////////////////////
/// 
/// 小数点以下1桁を四捨五入
/// 
/// ////////////////////////////////////
pub fn round_p1(param:f32)->String
{
    apis::round_p1(param)
}

////////////////////////////////////////
/// 
/// 小数点以下3桁を四捨五入
/// 
/// ////////////////////////////////////
pub fn round_p3(param:f32)->String
{
    apis::round_p3(param)
}




#[cfg(test)]
mod test_floor_seven{

   use super::*;

    #[test]
    fn test_no1(){
      let expect_value:u32=0;
      let param:u32=0;
     
      let result=floor_seven(param);

      assert_eq!(expect_value,result);


    }

    #[test]
    fn test_no2(){
        let expect_value:u32=7;
        let param:u32=9;
       
        let result=floor_seven(param);
  
        assert_eq!(expect_value,result);
  
  
      }

      #[test]
      fn test_no3(){
        let expect_value:u32=7;
        let param:u32=7;
       
        let result=floor_seven(param);
  
        assert_eq!(expect_value,result);
  
  
      }
}

#[cfg(test)]
mod test_round_p1
{
    use super::*;
    #[test]
    fn test_no1(){
        let expect_value:String=String::from("0.0");
        let param:f32=0.0;

        let result=round_p1(param);
        assert_eq!(expect_value,result);
    }

    #[test]
    fn test_no2(){
        let expect_value:String=String::from("9.2");
        let param:f32=9.24;

        let result=round_p1(param);
        assert_eq!(expect_value,result);
    }

    
    #[test]
    fn test_no3(){
        let expect_value:String=String::from("9.3");
        let param:f32=9.25;

        let result=round_p1(param);
        assert_eq!(expect_value,result);
    }

    #[test]
    fn test_no4(){
        let expect_value:String=String::from("9.2");
        let param:f32=9.245;

        let result=round_p1(param);
        assert_eq!(expect_value,result);
    }
}

mod test_round_p3
{
    use super::*;

    #[test]
    fn test_no1(){
        let expect_value:String=String::from("0.000");
        let param:f32=0.0;

        let result=round_p3(param);
        assert_eq!(expect_value,result);
    }

    #[test]
    fn test_no2(){
        let expect_value:String=String::from("9.212");
        let param:f32=9.2124;

        let result=round_p3(param);
        assert_eq!(expect_value,result);
    }

    #[test]
    fn test_no3(){
        let expect_value:String=String::from("9.213");
        let param:f32=9.2125;

        let result=round_p3(param);
        assert_eq!(expect_value,result);
    }

    
    #[test]
    fn test_no4(){
        let expect_value:String=String::from("9.212");
        let param:f32=9.21249;

        let result=round_p3(param);
        assert_eq!(expect_value,result);
    }
}