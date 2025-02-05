
//////////////////////////////////////////
/// 
/// ファイルユーテリティ
/// 
/// /////////////////////////////////////
use std::path::Path;

///フォルダ判定
pub fn is_folder(path:&str)->bool
{
  let judge=path.ends_with("/");

  return judge;
}

///親フォルダ存在チェック
pub fn is_parent_exists(path:&str)->bool
{
    let path_obj=Path::new(path);
    let parent=path_obj.parent()
                              .unwrap();
    return parent.exists();
}