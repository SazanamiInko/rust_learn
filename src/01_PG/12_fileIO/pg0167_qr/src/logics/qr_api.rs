/////////////////////////////////////
/// 
/// QRコードAPI
///
/// /////////////////////////////////
use std::error::Error;
use qrcode::QrCode;
use image::Luma;

///QRコードを発行する
pub fn publish_qr(path:&str,message:&str)->Result<(),Box<dyn Error>>
{
    //QRコードの作成
    let qr=QrCode::new(message)?;

    //QRコードの画像生成
    let image_qr=qr.render::<Luma<u8>>().build();

    //PNGファイルに保存
    image_qr.save(path)?;

    return Ok(());
}