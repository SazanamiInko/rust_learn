//////////////////////////////
/// 
/// パスの管理
/// 
/// ///////////////////////////
use std::path::Path;

///ベースとなるパスを返す。
pub fn get_path()->String
{
    return r"C:\Users\Public\Documents\data\Rust\tr0016".to_string();
}

///本部のパスを返す
pub fn get_center_path(base_path:&str)->String
{
    let base_path=Path::new(base_path);
    let center_path=base_path.join("center");
    return center_path.to_str().unwrap().to_string();
}

//支店の来店記録のパスを返す
pub fn get_come_path(base_path:&str,branch_name:&str)->String
{
    let base_path=Path::new(base_path);
    let branch_come_file=format!("{}/come.csv",branch_name);
    let branch_come_path=base_path.join(branch_come_file);
    return branch_come_path.to_str().unwrap().to_string();
}

///結果ファイルのパスを返す
pub fn get_result_path(center_path:&str)->String
{
    let center_path=Path::new(center_path);
    let result_path=center_path.join("result.csv");
    return result_path.to_str().unwrap().to_string();
}


///1000円券名簿のパスを返す
pub fn get_pay1000_path(center_path:&str)->String
{
    let center_path=Path::new(center_path);
    let pay1000_path=center_path.join("pay1000.csv");
    return pay1000_path.to_str().unwrap().to_string();
}

//500円券名簿のパスを返す
pub fn get_pay500_path(center_path:&str)->String
{
    let center_path=Path::new(center_path);
    let pay500_path=center_path.join("pay500.csv");
    return pay500_path.to_str().unwrap().to_string();
}

//支店リストのパスを返す
pub fn get_branch_list_path(center_path:&str)->String
{
    let center_path=Path::new(center_path);
    let branch_list_path=center_path.join("branch_list.txt");
    return branch_list_path.to_str().unwrap().to_string();

}