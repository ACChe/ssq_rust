use std::fs::File;
use std::path::Path;
extern crate rand;
use rand::Rng;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
struct SSQJson {
    issue:  u16,
    red1:   u8,
    red2:   u8,
    red3:   u8,
    red4:   u8,
    red5:   u8,
    red6:   u8,
    blue:   u8
}

fn get_ssq_from_file () -> Vec<SSQJson>{
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

    loop {
        let rssq = SSQ::new();
        for r in &ssqjson {
            let r1 = &ssqjson_to_ssq(&r);
            if compare_ssq_red(&r1.reds, &rssq.reds) && r1.blue == rssq.blue   {
                println!("THE LUCKIED SSQ IS : {:?}", r1);
                return rssq
            }  else {
                println!("THE LUCKY SSQ IS : {:?}", rssq);
                return rssq;
            }
        }
    }

}

#[derive(Debug)]
pub struct SSQ {
    reds:   Vec<u8>,
    blue:   u8
}

fn ssqjson_to_ssq (sj: &SSQJson) -> SSQ {
    let mut result: Vec<u8> = Vec::new();
    result.push(sj.red1);
    result.push(sj.red2);
    result.push(sj.red3);
    result.push(sj.red4);
    result.push(sj.red5);
    result.push(sj.red6);
    SSQ {
        reds: result,
        blue: sj.blue
    }
}

impl SSQ {
    pub fn new() -> Self {
        let reds = gem_red_numbers();
        let blue = gem_blue_number();
        let ssq = Self {
            reds: reds,
            blue: blue
        };
        ssq
    }
}

fn gem_red_numbers() -> Vec<u8>{
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

fn gem_blue_number() -> u8{
    let mut rng = rand::thread_rng();
    let rnt = rng.gen_range(1..17);
    rnt
}

