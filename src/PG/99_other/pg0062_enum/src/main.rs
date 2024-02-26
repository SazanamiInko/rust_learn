////////////////////////////
/// 
/// 列挙
/// 
///////////////////////////

//ライブラリ
use rand::Rng;
use std::fs::OpenOptions;
use std::io::prelude::*;

//列挙

//お告げ
enum Otuge
{
    Veryhutopara,
    Hutopara,
    Common,
    Goenn
}

//メイン関数
fn main() {

    let osaisen=get_money(get_otuge());
    _=post_saisenn(osaisen);

    println!("お賽銭を入れました。");
}

//お告げを取得する
fn get_otuge()->Otuge
{
    let mut gen=rand::thread_rng();
    let otuge=gen.gen_range(1..100);

    match otuge
    {
        otuge if otuge>95=>Otuge::Veryhutopara,
        otuge if otuge>90=>Otuge::Hutopara,
        otuge if otuge>5=>Otuge::Common,
        _ =>Otuge::Goenn,
    }
}

//お金を取り出す
fn get_money(otuge:Otuge)->String
{
    match otuge
    {
        Otuge::Veryhutopara=>"10000円".to_string(),
        Otuge::Hutopara=>"1000円".to_string(),
        Otuge::Common=>"10円".to_string(),
        Otuge::Goenn=>"5円".to_string(),        
    }
}

//お賽銭を入れる
fn post_saisenn(osaisen:String)->std::io::Result<()>
{
    let path=r"C:\Users\Public\Documents\Data\saisenbox.txt".to_string();

    let mut file =OpenOptions::new()
    .append(true)
    .open(path)?;

    _=file.write(osaisen.as_bytes());
    _=file.write("\n".as_bytes());

    Ok(())

}