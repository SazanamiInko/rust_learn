///////////////////////////////
/// 
/// ZIPAPI
/// 
/// ////////////////////////////
use std::{fmt::Error, fs::File, io::BufWriter, option, path::Path};
use zip::write::FileOptions;

///Zip圧縮
pub fn compress_zip(path:&str)->Result<(),Error>
{
  //ZIP拡張子
  let zip_ext=String::from("zip");
  //ZIPファイル名
  let foldername=get_folder_name(path);
  let zipname=format!("{}.{}",foldername,zip_ext);

  //空ファイルを作成
  let dest_path=r"C:\Users\Public\Documents\Data\Rust\work\pg0163\out";
  let zip_ful_path=format!("{}/{}",dest_path,zipname);
  let zip_file=File::create(zip_ful_path).expect("ファイル作成に失敗しました");

  //ファイルストリーム作成
  let mut writer=BufWriter::new(zip_file);
  let mut zip_writer=zip::ZipWriter::new(writer);

  //ZIPオプション
  //ユーザーは全権限を持ち、他は読み取りのみ
  let file_option:FileOptions<()> =FileOptions::default()
                                         .compression_method(zip::CompressionMethod::Stored)
                                         .unix_permissions(0o755);

  //ZIP圧縮
  _=directry_compress(path,
                    &mut zip_writer, 
                    file_option,
                    path);
                    
  return Ok(());
}

///パスからフォルダ名を取得
fn get_folder_name(path:&str)->String
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


///ディレクトリのZIP圧縮
fn directry_compress(dir: &str,
                     zip_writer: &mut zip::ZipWriter<BufWriter<File>>,
                    options: FileOptions<()>,
                    base_dir: &str,)->zip::result::ZipResult<()>
  {
    //圧縮対象にパス
    let path = Path::new(dir);
    //出力先のパス
    let base_path = Path::new(base_dir);

    if path.is_dir()
    {
      //ディレクトリの一覧を取得して繰り返す
      for entry in path.read_dir()?
      {
        let entry=entry.unwrap();
        let path=entry.path();

        //パスの重複部分を削除
        let name=path.strip_prefix(base_path)
                           .unwrap()
                           .to_str()
                           .unwrap();

        //対象がファイルだった場合
        if path.is_file()
        {
          //ZIPへ入れる内容の取得
          let mut file_p=File::open(&path)
                                    .unwrap();
          //ZIPの入口作成
          zip_writer.start_file(name, options)
                    .unwrap();

          //ZIPへコピー
          _=std::io::copy(&mut file_p, zip_writer);
        }
        else if path.is_dir(){
            
            //ZIPにディレクトリを追加
            _=zip_writer.add_directory(name, options);
            //再起呼び出しで子のディレクトリもZIP化
            _=directry_compress(path.to_str().unwrap(), 
                               zip_writer, 
                               options, 
                               base_dir);
        }
      }
    }

    return Ok(());
  }