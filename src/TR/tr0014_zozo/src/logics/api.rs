/////////////////////////////////////
/// 
/// API
/// 
/// /////////////////////////////////
use std::error::Error;
use unicode_normalization::UnicodeNormalization;

///全角→半角
pub fn convert_to_han(source:&str)->String
{
    return  source.nfkc().collect();
}

//Viewerの作成
pub fn open_brouser(url:&str)->Result<(),Box<dyn Error>>
{
    webbrowser::open(url)?;

    return Ok(());
}
