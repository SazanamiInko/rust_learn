////////////////////////////
/// 
/// マッチガード
/// 
///////////////////////////

//ライブラリ
use rand::Rng;
use std::fs::OpenOptions;
use std::io::prelude::*;

//定数
const OTUGE_VERRYHUTIOARRA:i32=0;
const OTUGE_HUTIOARRA:i32=1;
const OTUGE_COMMON:i32=2;
const OTUGE_HUTUU:i32=3;
const OTUGE_GOENN:i32=4;


//メイン関数
fn main() {

    let osaisen=get_money(get_otuge());
    _=post_saisenn(osaisen);

    println!("お賽銭を入れました。");
}

//お告げを取得する
fn get_otuge()->i32
{
    let mut gen=rand::thread_rng();
    let otuge=gen.gen_range(1..100);

    match otuge
    {
        otuge if otuge>95=>OTUGE_VERRYHUTIOARRA,
        otuge if otuge>90=>OTUGE_HUTIOARRA,
        otuge if otuge>5=>OTUGE_COMMON,
        _ =>OTUGE_GOENN,
    }
}

//お金を取り出す
fn get_money(otuge:i32)->String
{
    match otuge
    {
        OTUGE_VERRYHUTIOARRA=>"10000円".to_string(),
        OTUGE_HUTIOARRA=>"1000円".to_string(),
        OTUGE_COMMON=>"10円".to_string(),
        OTUGE_HUTUU=>"10円".to_string(),
        OTUGE_GOENN=>"5円".to_string(),
        _=>"10円".to_string()
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