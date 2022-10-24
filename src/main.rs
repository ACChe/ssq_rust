mod ssq;
pub fn main() {
    let ssq_pool = ssq::get_ssq_from_file();
    let ssq = ssq::gen_by_user(5, 33, 6, &ssq_pool);
    println!("ssq = {:?}", ssq);
}
