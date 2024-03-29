
/////////////////////////////////
///
/// ファイルが無かったら作成
/// 
//////////////////////////////////
use std::{fs::File, io::ErrorKind};
use std::fs::OpenOptions;
use std::io::Write;

//メイン関数
fn main() {
    let path=r"C:\Users\Public\Documents\Data\Rust\work\pg0097\sousi.txt";

    let mut file=open_file(path).unwrap();
    let data="春は曙".as_bytes();

    file.write_all(data).expect("エラーが発生しました");

    println!("ファイルを書いたわよ");
}

//ファイルを開く
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