mod logics;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[cfg(test)]
mod test_floor_seven{
    use crate::logics::apis;

    #[test]
    fn test_no1(){
      let expect_value:u32=0;
      let oaram:u32=0;
     
      let result=apis::floor_seven(oaram);

      assert_eq!(expect_value,result);


    }

    #[test]
    fn test_no2(){
        let expect_value:u32=7;
        let oaram:u32=9;
       
        let result=apis::floor_seven(oaram);
  
        assert_eq!(expect_value,result);
  
  
      }

      #[test]
      fn test_no3(){
        let expect_value:u32=7;
        let oaram:u32=7;
       
        let result=apis::floor_seven(oaram);
  
        assert_eq!(expect_value,result);
  
  
      }
}

#[cfg(test)]
mod test_round_p1
{
    use crate::logics::apis;

    #[test]
    fn test_no1(){
        let expect_value:String=String::from("0.0");
        let oaram:f32=0.0;

        let result=apis::round_p1(oaram);
        assert_eq!(expect_value,result);
    }

    #[test]
    fn test_no2(){
        let expect_value:String=String::from("9.2");
        let oaram:f32=9.24;

        let result=apis::round_p1(oaram);
        assert_eq!(expect_value,result);
    }

    
    #[test]
    fn test_no3(){
        let expect_value:String=String::from("9.3");
        let oaram:f32=9.25;

        let result=apis::round_p1(oaram);
        assert_eq!(expect_value,result);
    }

    #[test]
    fn test_no4(){
        let expect_value:String=String::from("9.2");
        let oaram:f32=9.245;

        let result=apis::round_p1(oaram);
        assert_eq!(expect_value,result);
    }
}

mod test_round_p3
{
    use crate::logics::apis;

    #[test]
    fn test_no1(){
        let expect_value:String=String::from("0.000");
        let oaram:f32=0.0;

        let result=apis::round_p3(oaram);
        assert_eq!(expect_value,result);
    }

    #[test]
    fn test_no2(){
        let expect_value:String=String::from("9.212");
        let oaram:f32=9.2124;

        let result=apis::round_p3(oaram);
        assert_eq!(expect_value,result);
    }

    #[test]
    fn test_no3(){
        let expect_value:String=String::from("9.213");
        let oaram:f32=9.2125;

        let result=apis::round_p3(oaram);
        assert_eq!(expect_value,result);
    }

    
    #[test]
    fn test_no4(){
        let expect_value:String=String::from("9.212");
        let oaram:f32=9.21249;

        let result=apis::round_p3(oaram);
        assert_eq!(expect_value,result);
    }
}