////////////////////////////////
/// 
/// ビープ音
/// 
////////////////////////////////
use std::thread;
use std::time::Duration;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::System::Diagnostics::Debug::*;

//メイン関数
fn main() {
    unsafe
    {
        MessageBeep(MB_OK)
        .expect("ごめんなさい");
        
        //1秒待つ
        thread::sleep(Duration::from_millis(1000));
        
        MessageBeep(MB_ICONERROR)
        .expect("やっぱごめんなさい");
    }
}
