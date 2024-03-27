/////////////////////////////////
///
/// 文字の加算
/// 
//////////////////////////////////

//メイン関数
fn main() {

    let char_array:[char;26]=['A';26];

    for n in 0..char_array.len()
    {
        char_array[n]=char_array[n]+(1) as char ;
    }

    for ch in char_array
    {
        print!("{},",ch);
    }
    println!("");
}
