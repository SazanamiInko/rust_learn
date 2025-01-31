////////////////////////////
///  
/// 7の環
/// 
/// ///////////////////////
use std::ops::Add;

//データ定義
#[derive(Debug, Clone, Copy)]
pub struct CircleSeven
{
    ///数
    num:u16
}

///実装
impl CircleSeven
{
    ///コンストラクタ
    pub fn from(base:u16)->Self
    {
        let m=base%7;
        let mut seed:u16=0;

        if m==0
        {
            seed=7;
        }
        else
        {
            seed=m;
        }

        return CircleSeven{num:seed};
    }

    ///表示
    pub fn display(self)->()
    {
        println!("{}",self.num);
    }

    ///数を取得
    pub fn get(self)->u16
    {
        return self.num;
    }
}

//+演算子の実装
impl Add for CircleSeven
{
    ///型定義
    type Output = Self;

    ///+演算子
    fn add(self,another:Self)->Self
    {
        return CircleSeven::from(self.num+another.num);
    } 
}
