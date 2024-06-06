////////////////////////////////////////////
/// 
/// メッセージボックス
/// 
/// /////////////////////////////////////////
use windows::{core::*, Win32::UI::WindowsAndMessaging::*};

///メイン関数
fn main() {
    unsafe
    {
        let title=w!("肩にカー助");
        let message=w!("わたしはカー子");
        
         _= MessageBoxW(None,
                    message,
                    title,
                    MB_OK);
    }
}
