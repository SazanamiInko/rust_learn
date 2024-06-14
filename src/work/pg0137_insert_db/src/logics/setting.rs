//////////////////////////////////
/// 
/// 設定ファイル
/// 
/// ///////////////////////////////

///設定ファイル
pub struct Setting{

    ///店舗番号
    pub shop:i32,
    ///サーバー情報,
    pub server:ServerInfo,
    ///ロッカー情報
    pub locker:LockerInfo

}

///サーバー情報
pub struct ServerInfo{

}

//ロッカー情報
pub struct LockerInfo{

    pub path:String
}