////////////////////////////////////////////
///
/// 株主構造体
/// 
/// ////////////////////////////////////////

///株主
pub struct Stocker
{
    ///株主番号
    id:String,
    ///取得年
    enter_year:u32,
    ///株数
    stocks:u32
}

///実装
impl Stocker
{
    ///実装コンストラクタ
    pub fn new(id:&str,
               enter_year:u32,
               stocks:u32)->Self
    {
        return Stocker{
            id:id.to_string(),
            enter_year:enter_year,
            stocks:stocks
        }
    }
}

