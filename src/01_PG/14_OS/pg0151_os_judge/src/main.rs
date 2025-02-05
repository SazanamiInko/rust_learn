//////////////////////////////////////////////////
/// 
/// OS判定
/// 
/// ///////////////////////////////////////////////

//定数
const WIN:u32=1;
const LINUX:u32=2;
const OTHER:u32=3;

///メイン関数
fn main() {
    
    let os= judge_os();

    if os==WIN
    {
        println!("OSはWindowsです");
    }
    else if os==LINUX {
        println!("OSはLinuxです");
    }
}

///OS判定
fn judge_os()->u32
{
    if cfg!(target_os= "windows")
    {
        return WIN;
    }

    if cfg!(target_os= "linux")
    {
        return LINUX;
    }

    return OTHER;

}
