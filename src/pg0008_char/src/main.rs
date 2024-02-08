/////////////////////////////////
///
///文字型
/// 
//////////////////////////////////

/// . メイン関数

fn main() {
    //文字型
    let char_val :char='a';

    //文字列型
    let str_ptr:&str="Pointer";
    let string_val:String="String".to_string();

    //表示
    println!("char_val={}",char_val);
    println!("str_ptr={}",str_ptr);
    println!("string_val={}",string_val);
}
