//////////////////////////////////
/// 
/// Ferica物理削除コンポネント
/// 
/// //////////////////////////////
use crate::logics::components::dao::master_card::MasterCard;
use crate::logics::components::CountComponent;
use mysql::Transaction;
use std::error::Error;

///Ferica物理削除コンポネント
///データ定義
pub struct RemoveFericaComponent{}

///実装
impl RemoveFericaComponent
{
    ///コンストラクタ
    pub fn new()->Self
    {
        return RemoveFericaComponent{} ;
    }
}

///コンポネント実装
impl CountComponent for RemoveFericaComponent
{
    ///パラメータ検査
    fn check_param(&self)->Result<(),Box<dyn Error>>
    {
        return Ok(());
    }

    ///相関チェック
    #[warn(unused_variables)]
    fn check_logical(&self,tran:&mut Transaction)->Result<(),Box<dyn Error>>
    {
        return Ok(());
    }

      ///実行
      fn execute(&self,tran:&mut Transaction)->Result<u32,Box<dyn Error>>
      {
        let ret=MasterCard::delete(tran)?; 
        return Ok(ret);
      }
}   