/////////////////////////////////
/// 
/// メンテナンスFerica管理
/// 
/// /////////////////////////////
use mysql::TxOpts;
use crate::logics::components::Component;
use crate::logics::components::add_ferica_component::AddFericaComponent;
use crate::logics::components::delete_ferica_component::DeleteFericaComponent;
use crate::logics::components::common::setting;
use crate::logics::components::dao::common::connection;
use std::error::Error;

///管理Fericaの登録
pub fn add_ferica(new_ferica:&str,add_ferica:&str)->Result<(),Box<dyn Error>>
{

    //コンポーネントの生成
    let component=AddFericaComponent::new(new_ferica,add_ferica);
    //パラメータチェック
    component.check_param()?;

    //設定ファイルの読み込み
    let setting=setting::load();
    //トランザクションの作成
    let con=connection::from_config(&setting.server);
    let mut pool=con.get_connection();
    let mut tran=pool.start_transaction(TxOpts::default())?;
   
  
   component.check_logical(&mut tran)?;
   _=component.execute(&mut tran);
   _=tran.commit();
    return Ok(());
}

///フェリカの削除
pub fn delete_ferica(delete_fericard:&str,applicant_ferica:&str)->Result<(),Box<dyn Error>>
{
    //コンポーネントの生成
    let component=DeleteFericaComponent::new(delete_fericard,applicant_ferica);

    component.check_param()?;

     //設定ファイルの読み込み
     let setting=setting::load();
     //トランザクションの作成
     let con=connection::from_config(&setting.server);
     let mut pool=con.get_connection();
     let mut tran=pool.start_transaction(TxOpts::default())?;

    component.check_logical(&mut tran)?;
    _=component.execute(&mut tran);
    _=tran.commit();
    return Ok(());

}