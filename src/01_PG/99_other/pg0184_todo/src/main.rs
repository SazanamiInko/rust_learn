/////////////////////////////////////////
/// 
/// TODO警告
/// 
/// /////////////////////////////////////


///未実装の関数
#[deprecated= "TODO: この関数は後で実装する予定です"]
pub fn func()
{
    // TODO: 警告は行単位ではかけられない。関数単位でかけます。

}

fn main() {
   
    println!("警告メッセージは関数を呼ぶことでかけられます");
    func();
}
