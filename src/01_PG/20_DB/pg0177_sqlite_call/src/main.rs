////////////////////////////////
/// 
/// SQLiteのバージョン確認
/// 
/// ////////////////////////////
mod logics;

///主関数
fn main() {
    let result=logics::system_apis::get_db_version();

  match result {

    Ok(version)=>{println!("SQLiteのバージョンは{}",version);}
    Err(e)=>
    {
        println!("{}",e);
        println!("SQLiteのバージョンの取得に失敗しました。")
    }
      
  }  
    
    
}
