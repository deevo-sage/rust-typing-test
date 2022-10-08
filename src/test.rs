use rand::Rng;
use std::{
    io,
    time::{Duration, SystemTime},
};

pub struct Info {
    count: u8,
    words: Vec<String>,
}

pub struct Test {
    input: String,
    start: SystemTime,
    end: SystemTime,
}

#[derive(Debug)]
pub struct Result {
    wpm: i8,
    time_taken: Duration,
}

static WORDS: [&str; 14] = [
    "apple",
    "mango",
    "dragon",
    "comedy",
    "as",
    "a",
    "the",
    "happy",
    "sad",
    "maple",
    "tree",
    "nice",
    "adjective",
    "haloween",
];

pub fn start() -> Info {
    println!("Monkay Type mein aapka swagatam!!!");
    println!("How Long? ");
    let mut num: String = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");
    println!("num: {:?}", num);
    let count = num.trim().parse().unwrap();
    Info {
        count,
        words: get_words(count),
    }
}

pub fn get_words(num: u8) -> Vec<String> {
    let len = WORDS.len();
    let mut i = 0;
    let mut arr: Vec<String> = Vec::new();
    loop {
        let x = rand::thread_rng().gen_range(0..len);
        arr.push(String::from(WORDS[x]));
        i += 1;
        if i == num {
            break;
        }
    }
    arr
}

pub fn take_input(info: &Info) -> Test {
    println!("{:?}", info.words.join(" "));
    println!("Start typing idiot");
    let mut input = String::new();
    println!("press enter to start");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let start = std::time::SystemTime::now();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let end = std::time::SystemTime::now();
    println!("end: {:?}", end);

    return Test { input, start, end };
}

pub fn get_result(input: Test, info: Info) -> Result {
    Result {
        wpm: 100,
        time_taken: input.end.duration_since(input.start).unwrap(),
    }
}
