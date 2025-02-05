///////////////////////////////
/// 
/// ZIPAPI
/// 
/// ////////////////////////////
use crate::logics::common::file_util;
use crate::logics::common::zip_util;
use std::fmt::Error;
use zip::write::FileOptions;

///Zip圧縮
pub fn compress_zip(source_path:&str,
                    dest_path:&str)
                    ->Result<(),Error>
{
  //ZIP拡張子
  let zip_ext=String::from("zip");

  //ZIPファイル名
  let foldername=file_util::get_folder_name(source_path);
  let zipname=format!("{}.{}",foldername,zip_ext);

  //空ファイルを作成
  let zip_ful_path=format!("{}/{}",dest_path,zipname);
  let zip_file=file_util::create_empty_file(zip_ful_path.as_str());

  //ファイルストリーム作成
  let mut zip_writer=zip_util::create_zip_stream(zip_file);

  //ZIPオプション
  //ユーザーは全権限を持ち、他は読み取りのみ
  let file_option:FileOptions<()> = FileOptions::default()
                                    .compression_method(zip::CompressionMethod::Stored)
                                    .unix_permissions(0o755);

  //ZIP圧縮
  _=zip_util::directry_compress(source_path,
                    &mut zip_writer, 
                    file_option,
                    source_path);
                    
  return Ok(());
}



