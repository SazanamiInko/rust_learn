////////////////////////////
/// 
/// ファイル出力
/// 
/// /////////////////////////

use std::{fs::File, io::ErrorKind};
use std::fs::OpenOptions;
use std::io::Write;

///ファイル出力
pub fn write(record:&super::record::Mesure)->Result<bool,String>
{
    let path=r"C:\Users\Public\Documents\Data\Rust\work\pg0117\mesure.txt";
    let mut file=open_file(path).unwrap();
                      
    let json=serde_json::to_string(record)
                    .unwrap();

    //所有権エラー対策
    let json_org=json;

    for line in json_org.lines()
    {
        let writedata=line.as_bytes();
        file.write(writedata).expect("書き込み失敗しました");
        _=file.write("\n".as_bytes());
    }
    return Ok(true);
}   

///ファイルを開く
fn open_file(path:&str)->Result<File,std::io::Error>
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