//////////////////////////////////
/// 
/// Linux表示
/// 
/// ///////////////////////////////
use super::traits::viewer::Viewer;

///LinuxViewer
pub struct LinuxViewer{

    url:String
}

///トレイトの実装
impl Viewer for LinuxViewer{

    ///コンストラクタ
    fn new(url:&str)->Self {
        return LinuxViewer{
            url:String::from(url),
        };
    }

    ///表示
    fn View(&self) {
        
    }
}