////////////////////////
/// 
/// 定数
/// 
/// ////////////////////

///定数構造体
pub struct Define
{
    
}

///実装
impl Define
{
    ///パスの取得
    pub fn get_path(&self)->String
    {
        return r"C:\Users\Public\Documents\Data\Rust\work\pg0117\mesure.txt".to_string();
    }  
} 

///コンストラクタ
pub fn create()->Define
{
    return Define{};
}