/////////////////////////////////////
/// 
/// ロッカーモジュール
/// 
/// /////////////////////////////////

///開閉記録
pub struct OpenCloseLog
{
    ///ロッカー番号
    locker_no:String,
    ///ロッカー開扉時間
    open_time:String,
    ///ロッカー開扉時のバーコード
    open_barcode:String,
    ///ロッカー閉扉時間
    close_time:String,
    ///ロッカー閉扉時のバーコード
    close_barcode:String
}

///実装
impl OpenCloseLog{

    ///商品コードの取得
    pub fn get_goods_code(&self)->String
    {
        let goods_code=self.open_barcode[START..END].to_string();
        return goods_code; 
    }
}

///コンストラクタ
fn create(locker_no:String,
    open_time:String,
    open_barcode:String,
    close_time:String,
    close_barcode:String)->OpenCloseLog
    {
        return OpenCloseLog{
            locker_no:locker_no,
            open_time:open_time,
            open_barcode,open_barcode,
            close_time,close_time,
            close_barcode,close_barcode
        }
    }

///ロッカー開閉記録再編集の読み取り
pub fn load_open_close_record()->vec<OpenCloseLog>
{
    
}