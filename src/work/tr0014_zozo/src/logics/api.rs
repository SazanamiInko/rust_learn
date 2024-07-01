
/////////////////////////////////////
/// 
/// API
/// 
/// /////////////////////////////////
use super::viewers::enums::os_type::{self, OS_type};
use super::viewers::traits::viewer::{self, Viewer};
use std::error::Error;
use unicode_normalization::UnicodeNormalization;

///全角→半角
pub fn convert_to_han(source:&str)->String
{
    return  source.nfkc().collect();
}

///OS判定
pub fn get_os()->OS_type
{
    if cfg!(target_os= "windows")
    {
        return OS_type::Windows;
    }

    if cfg!(target_os= "linux")
    {
        return OS_type::Linux;
    }

    return OS_type::Other;
}

//Viewerの作成
pub fn create_viewer(os_type:OS_type,url:&str)->Result<Viewer,Box<dyn Error>>
{
    


    return Ok(());
}
