/////////////////////////
///
/// ファイルリスト取得
///
/////////////////////////
use std::fs;

fn main() {
    let path=r"C:\Users\Public\Documents\Data\Rust";

    let filelist=fs::read_dir(path).unwrap();

    for file_entry in filelist
    {
        let dir=file_entry.unwrap();
        let name=dir.file_name();
        let type_name =dir.file_type().unwrap();

        if !type_name.is_dir()
        {
            println!("{:?}", name);
        }
    }
}
