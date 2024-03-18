////////////////////////////
/// 
/// ファイル振り分け
/// 
/// ////////////////////////

use chrono::prelude::{DateTime, Utc, Datelike };
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

//メイン関数
fn main() {
   //定数
   let UNDONE:String=String::from("未処理");
   let DONE:String=String::from("処理済み");

   //設定ファイルを読み込む
   let target_path=get_target_dir();
   let target=Path::new(&target_path);
  
   //未処理ファイルのパス取得
   let _undone_path=target.join(UNDONE);
   //処理のパスが取得
   let _done_path=target.join(DONE);

   let filelist=get_file_list(_undone_path.to_string_lossy().to_string());
  
   //1件ずつ行う
   for file in filelist
   {
     let work_path=format!("{}/{}",_undone_path.to_string_lossy().to_string(),file);

      //タイムスタンプの取得
      let time_stamp=get_time_stamp(&work_path);
      //ファイル名
      let name=Path::new(file.as_str()).file_name().unwrap().to_string_lossy().into_owned();
     

      //日付のフォルダがあるか
      let move_dir_path=_done_path.join(time_stamp);
      let str_path=move_dir_path.as_os_str().to_string_lossy().to_string();
      if !move_dir_path.is_dir()
      {
         if let Err(_e)=fs::create_dir(&str_path)
         {
            println!("{:?}",str_path);
            println!("{:?}",_e);
         }
      }
      
      
      let new_file_path=format!("{}/{}",str_path,name);

      //ファイル移動
      _=fs::rename(work_path, new_file_path);
   }
}

//設定ファイルを開いて対象となるフォルダを取得する
fn get_target_dir()->String
{
   let mut target="".to_string();

   let path=String::from(r"C:\Users\Public\Documents\setting\postman.txt");
   if let Ok(file)=File::open(path){
      let reader = BufReader::new(file);
  
      for line in reader.lines()
      {
         target=line.unwrap();
      }
   }
   else
    {
      panic!("設定ファイルが開けませんでした");    
   }
   return target;
}

//指定されたフォルダのファイル一覧を返す
fn get_file_list(path:String)->Vec<String>
{
   let mut targets:Vec<String> =Vec::new();
   if let Ok(file_list)=fs::read_dir(path){
     for file_result in file_list{
        if let Ok(file_item) = file_result {
         let name=file_item.file_name().to_string_lossy().to_string();
       
         targets.push(name);
        } 
     }
   }
   else {
      panic!("失敗しました");    
   }

   return targets;
}

//指定されたファイルのタイムスタンプを取得する
fn get_time_stamp(target_path:&String)->String
{
   let meta= fs::metadata(target_path).unwrap();

   let created= meta.created().unwrap();
   let datetime_created_utc : DateTime<Utc>=created.into();
  
  let yyyymmdd= format!("{}{:02}{:02}"
            ,datetime_created_utc.year()
            ,datetime_created_utc.month()
            ,datetime_created_utc.day() );
            return yyyymmdd;
}


