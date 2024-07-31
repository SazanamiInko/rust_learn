////////////////////////////////
/// 
/// バーコードAPI
/// 
/// ////////////////////////////
use std::error::Error;
use bardecoder;
use image::io::Reader as ImageReader;

///バーコード読み取り
pub fn read_barcode(path: &str) -> Result<String, Box<dyn Error>> {
    // バーコードが含まれている画像のパスを指定します
    let image_path = path;

    // 画像を読み込みます
    let img = ImageReader::open(image_path)
        .map_err(|e| Box::new(e) as Box<dyn Error>)?
        .decode()
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

    // bardecoderのデコーダーを作成します
    let decoder = bardecoder::default_decoder();

    // 画像からバーコードをデコードします
    let results = decoder.decode(&img);

    // 結果を収集します
    let mut text = String::new();
    for result in results {
        match result {
            Ok(barcode) => text.push_str(&barcode),
            Err(e) => eprintln!("Error decoding barcode: {:?}", e),
        }
    }

    // textが空でなければそれを返し、空ならエラーを返します
    if text.is_empty() {
        Err("No barcode found".into())
    } else {
        Ok(text)
    }
}