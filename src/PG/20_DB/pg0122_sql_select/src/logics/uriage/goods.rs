/////////////////////////////////////
/// 
/// 商品マスタ
/// 
/// //////////////////////////////////

///商品マスタ
pub struct Goods
{
    pub id:i32,
    pub name:String
}

///実装
impl Goods
{
    ///表示
    pub fn display(&self)
    {
        println!("{}:{}",self.id,self.name);
    }
}

///コンストラクタ
pub fn create(id:i32,name:String)->Goods
{
    return Goods{
        id:id,
        name:name
    }
}