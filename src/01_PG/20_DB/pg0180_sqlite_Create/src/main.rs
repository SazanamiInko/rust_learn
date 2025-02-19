////////////////////////////
/// 
/// SQLiteのDBファイル作成
/// 
/// /////////////////////////
mod logics;

use crate::logics::common_api;
use crate::logics::sqlite_api;

///主関数
fn main() {

    let filename=common_api::get_file_name();
    let result= sqlite_api::create_dbfile(&filename);
   
    match result {
        Ok(_)=>{   println!("SQLiteのDBファイルを作成しました");}
        Err(e)=>{
            println!("{:?}",e);
            println!("SQLiteのDBファイルを作成に失敗しました");}
    }
   
 
}
