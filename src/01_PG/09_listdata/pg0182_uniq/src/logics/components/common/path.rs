///////////////////////////
/// 
/// パス関係
/// 
/// ////////////////////////
use std::path::Path;

///顧客台帳のパスを返す。
pub fn get_path()->String
{
    return r"C:\Users\Public\Documents\data\Rust\pg0182".to_string();
}

///本部のパスを返す
pub fn get_main_path()->String
{
    return r"C:\Users\Public\Documents\data\Rust\pg0182\本部.txt".to_string();
}

///支店のパスを返す
pub fn get_branch_path(custom_path:&str,branch_name:&str)->String
{
    let custom_path=Path::new(custom_path);
    let branch_file=format!("{}.txt",branch_name);
    let branch_path=custom_path.join(branch_file);
    return branch_path.to_str().unwrap().to_string();
}