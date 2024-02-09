////////////////////////////////
///
///多重For
/// 
//////////////////////////////////

use std::string;


fn main() {
    let teachers=[["はつね","のぶ","しゅうへい","もうもう","牛炊"],["佐々木","ライオン丸","きみちゃん","たにし君","まゆずみ"]];
    let mut gakunen=1;
    let mut kumi=1;


    for gakunenteachers in teachers
    {
        for teacher in gakunenteachers
        {
            println!("{}年{}組:{}先生",gakunen,kumi,teacher);
            kumi+=1;
        }

        println!("");
        gakunen+=1;
        kumi=1;
    }

 }
 