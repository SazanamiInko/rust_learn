//////////////////////////////////////////////////
/// 
/// 文字コード指定
/// 
/// //////////////////////////////////////////////

use std::fs::File;
use std::io::Write;
use encoding::{Encoding, EncoderTrap};
use encoding::all::WINDOWS_31J; // Shift-JIS

///メイン関数
fn main() -> std::io::Result<()> {
    let path = r"C:\Users\Public\Documents\Data\Rust\work\pg0135\daizu.txt";
    let text = "あずき"; // 書き込みたいテキスト

    // Shift-JISにエンコード
    let encoded = WINDOWS_31J.encode(text, EncoderTrap::Strict)
        .expect("文字コードの変換に失敗しました");

    // ファイルに書き込む
    let mut file = File::create(path)?;
    file.write_all(&encoded)?;

    Ok(())
}