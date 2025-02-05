//////////////////////////////////
/// 
/// 相関検査
/// 
/// //////////////////////////////

pub trait LogicalVerify {
    
fn verify(&self,tran:&mut Transaction)->Result<(),Box<dyn std::error::Error>>;

}

