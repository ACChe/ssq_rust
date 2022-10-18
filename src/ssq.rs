use std::path::Path;
use std::{arch::aarch64::int32x2_t, fs::File};
extern crate rand;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SSQJson {
    // issue:  u16,
    red1: u8,
    red2: u8,
    red3: u8,
    red4: u8,
    red5: u8,
    red6: u8,
    blue: u8,
}

fn get_ssq_from_file() -> Vec<SSQJson> {
    let path = Path::new("ssq.json");
    let display = path.display();

    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let ssqs: Vec<SSQJson> = serde_json::from_reader(file).expect("error while reading");

    // println!("{:#?}", ssqs);
    // for ssq in ssqs.iter() {
    //     println!("{:#?}", ssq);
    // }

    ssqs
}

fn compare_ssq_red(raw: &Vec<u8>, rand: &Vec<u8>) -> bool {
    let mut raw1 = raw.clone();
    let mut rand1 = rand.clone();
    raw1.sort();
    rand1.sort();
    raw1 == rand1
}

pub fn get_random_ssq() -> SSQ {
    let ssqjson = get_ssq_from_file();
    let mut count = 0;

    loop {
        let rssq = SSQ::new();
        count = count + 1;
        for r in &ssqjson {
            let r1 = &ssqjson_to_ssq(&r);
            return if compare_ssq_red(&r1.reds, &rssq.reds) && r1.blue == rssq.blue {
                println!("THE LUCKIED SSQ IS : {:?}", r1);
                rssq
            } else {
                println!("Round : {:?}", count);
                println!("THE LUCKY SSQ IS : {:?}", rssq);
                rssq
            };
        }
    }
}

// pub fn get_random_ssq_with_count(number: i8) -> &'static Vec<SSQ> {
//     let ssqjson = get_ssq_from_file();
//
//     let &mut result: Vec<SSQ> = Vec::new();
//     let mut index = 0;
//
//     loop {
//         if index > number {
//             // println!("THE LUCKY SSQ IS : {:?}", &result);
//             return &result
//         }
//
//         let rssq = SSQ::new();
//         for r in &ssqjson {
//             let r1 = &ssqjson_to_ssq(&r);
//             if !compare_ssq_red(&r1.reds, &rssq.reds) && r1.blue == rssq.blue {
//                 // println!("THE LUCKIED SSQ IS : {:?}", r1);
//                 for r2  in result {
//                     if !compare_ssq_red(&r2.reds, &rssq.reds) && r2.blue == rssq.blue {
//                         result.push(rssq.clone());
//                         index = index + 1;
//                     }
//                 }
//             }
//         }
//     }
// }

#[derive(Debug, Clone)]
pub struct SSQ {
    reds: Vec<u8>,
    blue: u8,
}

fn ssqjson_to_ssq(sj: &SSQJson) -> SSQ {
    let mut result: Vec<u8> = Vec::new();
    result.push(sj.red1);
    result.push(sj.red2);
    result.push(sj.red3);
    result.push(sj.red4);
    result.push(sj.red5);
    result.push(sj.red6);
    SSQ {
        reds: result,
        blue: sj.blue,
    }
}

impl SSQ {
    pub fn new() -> Self {
        let reds = gen_red_numbers();
        let blue = gen_blue_number();
        let ssq = Self { reds, blue };
        ssq
    }
}

fn gen_red_numbers() -> Vec<u8> {
    let max = 6;
    let mut count = 0;
    let mut numbers = vec![0; 0];
    let mut rng = rand::thread_rng();
    while count < max {
        let rnt = rng.gen_range(1..34);
        if !numbers.contains(&rnt) {
            numbers.push(rnt);
            count = count + 1;
        }
    }
    numbers
}

fn gen_blue_number() -> u8 {
    let mut rng = rand::thread_rng();
    let rnt = rng.gen_range(1..17);
    rnt
}

// General an unorder sequence
fn gen_numbers_limit(max: u8) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    // produce a limited array
    let mut vec: Vec<u8> = (1..max).collect();
    // shuffle the array
    vec.shuffle(&mut rng);
    vec
}

// Divide the sequence
pub fn div_numbers_to_group(mut sequence: Vec<u8>, limit: u8) -> Vec<Vec<u8>> {
    let total: u8 = sequence.len() as u8 / limit;
    let mut result: Vec<Vec<u8>> = Vec::new();
    let mut index = 0;
    while index < total {
        let temp: Vec<u8> = sequence.drain(0..6).collect();
        result.push(temp);
        index += 1;
    }
    result
}

// General number by specify number
fn gen_by_specify_number(specify: u8, total: u8, limit: u8) -> Vec<Vec<u8>> {
    let mut result: Vec<Vec<u8>> = Vec::new();
    let pool = gen_numbers_limit(total);
    let mut lucky_numbers = div_numbers_to_group(pool, limit);
    let s = specify as usize;
    if specify < limit {
        let temp = lucky_numbers.drain(0..s).collect();
        println!("Lucky number(s) : {:?}", lucky_numbers);
        result = temp;
        result
    } else {
        println!("Lucky number(s) : {:?}", lucky_numbers);
        result
    }
}

// General numbers by user need
pub fn gen_by_user(wanted: u8, total: u8, limit: u8) -> Vec<Vec<u8>> {
    let mut result: Vec<Vec<u8>> = Vec::new();
    //  可以产生多少组
    let count = total / limit;
    //  取整的组数
    let wanted_divide = wanted / count;
    //  取整的余数
    let wanted_mod = wanted % count;

    let mut index = 0;
    while index < wanted_divide {
        let mut temp = gen_by_specify_number(wanted, total, limit);
        result.append(&mut temp);
        index += 1;
    }

    index = 0;
    while index < wanted_mod {
        let mut temp = gen_by_specify_number(wanted_mod, total, limit);
        result.append(&mut temp);
        index += 1;
    }
    result
}
