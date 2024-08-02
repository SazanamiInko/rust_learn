////////////////////////////////
/// 
/// QRAPI
/// 
/// ////////////////////////////
use std::error::Error;
use quircs::Quirc;

///QRコードのデコード
pub fn decode(path:&str)->Result<String,Box<dyn Error>>
{
    let mut text=String::from("");

    //画像の読み込み
    let image=image::open(path)?
    .to_luma8();
   
    //QRコード読み取りオブジェ
    let mut qr_obj = Quirc::default();
    //読み取り
    let codes=qr_obj.identify(image.width() as usize, 
                                        image.height() as usize, 
                                        &image);

    //データ展開
    for code in codes 
    {
      let code_result=code?;
      let code_data=code_result.decode()?;
      let text_record= std::str::from_utf8(&code_data.payload)?;
      text.push_str(text_record);

    }


    return Ok((text));
}