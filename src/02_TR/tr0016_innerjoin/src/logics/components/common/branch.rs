///////////////////////////////
/// 
/// 支店
/// 
/// ///////////////////////////
use std::error::Error;
use crate::logics::components::datamodel::come_datamodel::ComeDataModel;
use std::fs::File;

//来店記録の読み込み
pub fn load_come_file(path:&str)->Result<Vec<ComeDataModel>,Box<dyn Error>>
{
    let mut ret:Vec<ComeDataModel>=Vec::new();

    //来店記録の読み込み
    let file=File::open(path).unwrap();
    let mut reader =  csv::ReaderBuilder::new()
                      .has_headers(false)
                      .from_reader(file);
  
    //来店記録へ格納
    for result in reader.records()
    {
      let record = result.unwrap();
      let one_record=ComeDataModel::new(&record[0],&record[1]);
     ret.push(one_record);
    }
    
    return Ok(ret);

}