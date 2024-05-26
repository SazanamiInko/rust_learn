  ///////////////////////////////////
  /// 
  /// 給与 
  /// 
  /// ////////////////////////////////
  mod achievements;
  mod file_io;
  mod tax;
  use std::{fs::File, io::Write};

    ///給与を表す構造体
    pub struct Salally
    {
        ///社員番号
        pub no:String,

        ///実績
        pub achievement:achievements::Achievements,

        ///給与原泉
        pub source:i32,														
       
        ///税金
        pub tax:tax::Tax,

        ///控除後給与
        pub  amount:i32
    }

    ///実装
    impl Salally{

      ///文字出力
      pub fn get_info(&self)->String
      {
        return format!("{},{},{},{},{}\n"
                       ,self.no
                       ,self.source
                       ,self.tax.income
                       ,self.tax.invoice
                       ,self.amount);
      }

    }

    ///コンストラクタ
    pub fn create_salally(no:String,start:String,end:String)->Result<Salally,String>{
       let achievements=achievements::from(no.as_str(), start.as_str(), end.as_str());
       let source=get_source(&achievements.achievements);
       let tax=tax::from(&source);
       let amount=tax.get_deduction(source);

       return  Ok(Salally { no:no, 
                            achievement:achievements, 
                            source:source,
                            tax:tax,
                            amount:amount } );
    }

   ///時給
   const TIME_MONEY:i32=2500;

  ///源泉
  fn get_source(unit:&f32)->i32
  {
    let zikyu=TIME_MONEY as f32;
    let source=*unit*zikyu;
    return (source) as i32;
  }

  ///実績取得
 pub fn open_jisseki()->Vec<Salally>
 {
    let mut ret:Vec<Salally>=Vec::new();

    let path=file_io::get_jisseki_path();
    let file=File::open(path).unwrap();
    let mut reader =  csv::ReaderBuilder::new()
                      .has_headers(false)
                      .from_reader(file);
  
    for result in reader.records()
    {
      let record = result.unwrap();
      let salally=create_salally(record[0].to_string(),
                                                          record[1].to_string(), 
                                                          record[2].to_string())
                                                          .unwrap();
      ret.push(salally);
    }

    return ret;
 }

 ///ファイルの書き込む
 pub fn write_sallery(records:Vec<Salally>)->Result<(),String>
 {
    //ファイルを開く
    let path=file_io::get_sallery_path();
    let mut file=file_io::create_open_file(path.as_str()).unwrap();

    for record in records.iter()
    {
      let data=record.get_info();
      file.write(data.as_bytes())
          .expect("ファイル書き込みに失敗しました");
    }
    return Ok(());
 }