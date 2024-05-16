/////////////////////
/// 
/// モジュール2
/// 
/// /////////////////
mod api;
mod logics;

///メイン関数
fn main() {
    //外部サーバーからAPIを叩くていです
    api::api_a();
    api::api_b();
    
    //給与計算します
    logics::sallary::calc_sallary();
    //税金
    let amount:u32=2000;
    let sal=logics::sallary::create_sallery(&amount);
    logics::tax::report_tax(&sal.amount);
}
