//////////////////////////////
/// 
/// Viewerトレイト
/// 
/// ///////////////////////////

///Viewer
pub trait Viewer
{
    ///インスタンス化
    fn new(url:&str)->Self;

    ///表示する
    fn View(&self);
}