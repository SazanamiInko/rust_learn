//////////////////////////////
/// 
/// 売り上げ
/// 
/// ///////////////////////////
use log::{info, error};
use backtrace::Backtrace;
use std::panic;


///売り上げ計上
pub fn up_uriage(mode:&i32)
{
    info!("up_uriage:start");

    if let Err(e) =panic::catch_unwind(||
        {
            uriage(mode);
        }
    )
    {
        let backtrace=Backtrace::new();
        error!("{:?}\n{:?}",e,backtrace);
    }

    info!("up_uriage:end")
}

///売り上げ
fn uriage(mode:&i32)
{
    if *mode==2
    {
        panic!("まだそのモードは開発できません");
    }
}