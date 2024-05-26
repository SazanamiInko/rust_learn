////////////////////////////////////////////////////////
/// 
/// ファイル出力
/// 
/// /////////////////////////////////////////////////////
use std::{fs::File, io::ErrorKind};
use std::fs::OpenOptions;

//ファイルを開く
pub fn create_open_file(path:&str)->Result<File,std::io::Error>
{
    let res=OpenOptions::new()
    .append(true)
    .open(path);

    match res {
        Ok(ref _fp)=>{return res}
        Err(ref error) =>
        {
            if error.kind()==ErrorKind::NotFound
            {
                return  create_file(path);
            }
            else
             {
                return res;    
            }
        }      
    }
}

//ファイルをを作成する
fn create_file(path:&str)->Result<File,std::io::Error>
{
    let f=File::create(path);
    return f;
}

///実績ファイルのパス
pub fn get_jisseki_path()->String
{
    return r"C:\Users\Public\Documents\Data\Rust\work\tr0010\achievements.txt".to_string();
}

///給与ファイルのパス
pub fn get_sallery_path()->String
{
    return r"C:\Users\Public\Documents\Data\Rust\work\tr0010\salally.txt".to_string();
}