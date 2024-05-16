////////////////////////////
/// 
/// 給与構造体
/// 
/// ////////////////////////

///給与構造体 
pub struct Sallery
{
   pub amount:u32
}

///給与ロジック
impl Sallery
{
    ///メッセージ表示
    pub fn print_message(&self)
    {
        println!("給与モジュールのデータ層 ぼくの給料は{}AUD",self.amount);
    }
}

///給与構造体インスタンス
pub fn create_sallery(amount:u32) ->Sallery{
    return Sallery{amount:amount};
}