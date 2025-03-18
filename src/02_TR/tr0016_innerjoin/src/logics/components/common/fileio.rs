use std::fs::File;
use std::io::{BufRead, BufReader,BufWriter,Write};
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;

///ファイルを1行ずつ読み取って返す
pub fn read_line(path:&str)->Result<Vec<String>,Box<dyn Error>>
{
    let file=File::open(path)?;
    let reader=BufReader::new(file);

    let ret:Vec<String>=reader.lines()
    .map(|reacord|reacord.unwrap())
    .collect();

    return Ok(ret);
}

//ファイルを1行ずつ書いていく
pub fn write_line(path:&str,lines:Vec<String>)->Result<(),Box<dyn Error>>
{
    //ファイルを削除する
    if Path::new(path).exists() 
    {
        match fs::remove_file(path)
        {
            Ok(())=>{}
            Err(e)=>{return Err(Box::new(e));}
        } 
    }

    //ファイルを開く
    let file = OpenOptions::new()
    .write(true)
    .append(true)  // 追記モード
    .create(true)  // ファイルがなければ作成
    .open(path)?;

    let mut writer = BufWriter::new(file);

    lines.iter().for_each(|record|
        {
            _=writeln!(writer,"{}",record);        
        }
    );
    return Ok(());
}