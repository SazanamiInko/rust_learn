////////////////////////////////////////
/// 
/// DB:レコードの削除
/// 
/// /////////////////////////////////////
mod logics;
use logics::ferica_api;

///主関数
fn main() {
    println!("不要データ削除(フェリカカード)");
    let result=ferica_api::remove_ferica();

    match result
    {
        Ok(count)=>
        {
            println!("{}件のフェリカマスタカードを削除しました",count);
        }
        Err(e)=>
        {
            println!("{}",e);
            println!("フェリカマスタカードの削除に失敗しました");
        }
    }
}
