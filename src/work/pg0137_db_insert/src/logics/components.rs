////////////////////////////////////////////
/// 
/// コンポネント
/// 
/// ////////////////////////////////////////

//モジュール一覧
pub mod common;
pub mod dao;
pub mod verifys;
pub mod add_ferica_component;


use mysql::Transaction;
use std::error::Error;

///コンポネント
pub trait Component{

///パラメータチェック
fn check_param(&self)->Result<(),Box<dyn Error>>;

///相関チェック
fn check_logical(&self,tran:&mut Transaction)->Result<(),Box<dyn Error>>;

///実行
fn execute(&self,tran:&mut Transaction)->Result<(),Box<dyn Error>>;

}

