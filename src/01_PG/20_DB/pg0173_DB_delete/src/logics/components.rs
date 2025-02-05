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
pub mod delete_ferica_component;
pub mod remove_ferica_component;


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

///件数を返すコンポネント
pub trait CountComponent {

///パラメータチェック
fn check_param(&self)->Result<(),Box<dyn Error>>;

///相関チェック
fn check_logical(&self,tran:&mut Transaction)->Result<(),Box<dyn Error>>;

///実行
fn execute(&self,tran:&mut Transaction)->Result<u32,Box<dyn Error>>;

    
}

