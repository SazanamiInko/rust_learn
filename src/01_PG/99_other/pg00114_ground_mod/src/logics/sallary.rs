mod data;

///給与計算
pub fn calc_sallary() {
    let s:data::Sallery=data::create_sallery(1000);
    println!("給与計算をしてます");
    s.print_message();
}

pub fn create_sallery(amount:&u32)->data::Sallery
{
    return data::create_sallery(*amount);
}