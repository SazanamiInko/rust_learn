///////////////////////////////////////
/// 
/// Windows表示
/// 
/// /////////////////////////////////////
use super::traits::viewer::Viewer;

///windows表示
pub struct WindowsViewer{
    url:String
}

///トレイトの実装
impl Viewer for WindowsViewer{

    ///コンストラクタ
    fn new(url:&str)->Self {
        return WindowsViewer{
            url:String::from(url),
        };
    }

    ///表示
    fn View(&self) {
        
    }
}