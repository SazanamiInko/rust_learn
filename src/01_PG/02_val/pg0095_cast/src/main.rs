/////////////////////////////////
///
/// キャスト
/// 
//////////////////////////////////
use num::FromPrimitive;
use std::io::Write;
use std::fs::File;

//メイン関数
fn main() {
    let num=1234;
    let mut data:[u8;4]=[0;4];
    let len=data.len();
    let mut level=u32::from_usize(len).expect("");

    for n in 0..len 
    {
        let pick=pickup(level, &num);
        data[n]=pick as u8;
        level=level-1;
    }

    write(&data);
    println!("ファイルを書き込みました")
}

//桁フィルター
//pickup_level:抜き出したい桁
//target:対象となる数
fn pickup(pickup_level:u32,target:&i32)->i32{
   
   let work=target;
   let fillter:i32=10;
   let up_filtter:i32=fillter.pow(pickup_level);
   let row_filtter:i32=fillter.pow(pickup_level-1);
   
   return  (work%up_filtter)/row_filtter;
}

//ファイルを作成
fn write(data:&[u8])
{
    let path=r"C:\Users\Public\Documents\Data\Rust\work\pg0095\test.dat".to_string();
    let mut f = File::create(path).expect("ファイルを開けません");
    
    f.write_all(&data).expect("書き込みに失敗しました");
    
}
