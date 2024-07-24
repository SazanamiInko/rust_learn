//////////////////////////////////
/// 
/// ZIP_API
/// 
/// //////////////////////////////
use crate::logics::common::file_util;
use std::fs::{create_dir_all, File};
use std::io::{self, BufReader};
use std::error::Error;
use std::path::Path;
use zip::read::ZipArchive;

///ZIP展開
pub fn extract_zip(source_path:&str,dest_path:&str)->Result<(),Box<dyn Error>>
{
    //ZIPファイルを開く
    let zipfile=File::open(source_path)?;

    //ZIPを読み取り
    let reader=BufReader::new(zipfile);
    let mut zip_reader= ZipArchive::new(reader)?;

    //ZIPから読み取った内容すべて
    for index in 0..zip_reader.len()
    {
        let output_full_path:&str="";

        //ZIPファイルから要素取り出し
        let mut file_obj=zip_reader.by_index(index)?;

        let output_full_path = match file_obj.enclosed_name() {
            Some(path) => Path::new(dest_path).join(path),
            None => continue,
        };

        //解凍処理
        //フォルダだった場合
        let output_full_path_work=output_full_path.clone();
        if file_util::is_folder(file_obj.name())
        {
            //フォルダを作る
            _=create_dir_all(output_full_path_work);
        }
        //ファイルだった場合
        else {
            
            //親フォルダがない
            if let Some(parent)=output_full_path_work.parent()
            {
                if !parent.exists()
                {
                    //フォルダを作る
                    _=create_dir_all(output_full_path_work.clone());
               }
            }

            //ファイルコピー
            let mut out_file=File::create_new(output_full_path_work)?;
            _=io::copy(&mut file_obj,&mut out_file)?;
        }
    }

    return Ok(());
}