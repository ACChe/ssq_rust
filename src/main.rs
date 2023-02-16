mod ssq;

use std::io;
pub fn main() {
    println!("Please input how many lucky number you want:");

    let mut amount = String::new();

    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read line");

    let amount: u8 = amount.trim().parse().expect("Please type a number");

    let ssq_pool = ssq::get_ssq_from_file();
    let ssq = ssq::gen_by_user(amount, 33, 6, &ssq_pool);
    println!("ssq = {:?}", ssq);
}
