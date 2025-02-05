/////////////////////////////////////////////
/// 
/// ファイルユーテリティ
/// 
/// /////////////////////////////////////////
use std::path::Path;
use std::fs::File;

///パスからフォルダ名を取得
pub fn get_folder_name(path:&str)->String
{
    let mut ret=String::from("");
    let path_obj=Path::new(path);

    let folder_name=path_obj.file_name();

    if let  Some(buf)= folder_name {
        
       let buf2= buf.to_str();
       if let Some(folder_str)=buf2
       {
         ret=folder_str.to_string();
       }
    }

    return ret;
}

///空ファイル作成
pub fn create_empty_file(path:&str)->File
{
  let f=File::create(path)
            .expect("ファイル作成失敗しました。");
  return f;
}
