///////////////////////////////////
/// 
/// none判定
/// 
/// ///////////////////////////////

///メイン関数
fn main() {
    let no_none=Some(5);
    let yes_none:Option<i32>=None;

    judge(&no_none);
    judge(&yes_none)
}

///判定関数
fn judge(arg:&Option<i32>)
{
    if arg.is_none()
    {
        println!("値がnoneです。");
    }
    else {
        println!("値は{}",arg.unwrap());
    }
}
