mod api;
mod logics;

fn main() {
    api::api_a();
    api::api_b();
    logics::sallary::calc_sallary();
    let sa=logics::sallary::data::create_sallery(2000);
}
