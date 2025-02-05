////////////////////////////////////////////
/// 
/// ZIPユーテリティ
/// 
/// ////////////////////////////////////////
use std::{fs::File, io::BufWriter,path::Path};
use zip::write::FileOptions;

///ディレクトリのZIP圧縮
pub fn directry_compress(dir: &str,
                         zip_writer: &mut zip::ZipWriter<BufWriter<File>>,
                         options: FileOptions<()>,
                         base_dir: &str)
                         ->zip::result::ZipResult<()>
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

///ZIPストリーム作成
pub fn create_zip_stream(file:File)-> zip::ZipWriter<BufWriter<File>>
{
    let writer=BufWriter::new(file);
    return zip::ZipWriter::new(writer);
}
