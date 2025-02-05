//////////////////////////////////////////
///
/// バーコード発行
///
///////////////////////////////////////////
use barcoders::generators::image::Image;
use barcoders::sym::code128::Code128;
use std ::error::Error;

///バーコード用データの作成
pub fn make_a_mode_data(code:&str)->String
{
    let ret=format!("À{}",code);
    return ret;
}

///バーコード発行
pub fn publish_barcode(code:&str,dest:&str)->Result<(),Box<dyn Error>>
{
    const HEIGHT:u32=120;

    //バーコードオブジェの作成
    let barcode_obj=Code128::new(code)
                    .unwrap();
    //バーコード作成
    let encode_data=barcode_obj.encode();
    //Imageを作成
    let image_obj=Image::png(HEIGHT);
    //PNGに変換
    let png_image=image_obj.generate(&encode_data)?;
    //保存
    std::fs::write(dest, png_image)
             .unwrap();


    return Ok(());
}