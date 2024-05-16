mod sallery_file_io
{
    //実績ファイルを開く
fn open_jisseki()->Vec<salally::Salally>
{
    let mut ret:Vec<salally::Salally>=Vec::new();

    let path=r"C:\Users\Public\Documents\Data\Rust\work\tr0010\achievements.txt".to_string();
    let file=File::open(path).unwrap();
    let mut reader =  csv::ReaderBuilder::new()
                      .has_headers(false)
                      .from_reader(file);
  
    for result in reader.records()
    {
      let record = result.unwrap();
     
    }

    return ret;
}

}