////////////////////////////////////////////
/// 
/// コンポネント
/// 
/// ////////////////////////////////////////

//モジュール一覧
pub mod common;
pub mod dao;
pub mod resources;
pub mod get_discount_component;
pub mod find_discount_component;
pub mod verifys;

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

pub trait GetComponent<T> {
    ///パラメータチェック
fn check_param(&self)->Result<(),Box<dyn Error>>;

///相関チェック
fn check_logical(&self,tran:&mut Transaction)->Result<(),Box<dyn Error>>;

///実行
fn execute(&self,tran:&mut Transaction)->Result<Vec<T>,Box<dyn Error>>;

}

