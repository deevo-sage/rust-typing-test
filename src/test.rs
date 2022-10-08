use colored::Colorize;
use rand::Rng;
use std::{
    env, io, thread,
    time::{Duration, SystemTime},
};

pub struct Info {
    count: usize,
    words: Vec<String>,
}

pub struct Test {
    input: String,
    start: SystemTime,
    end: SystemTime,
}

#[derive(Debug)]
pub struct Result {
    wpm: f64,
    raw: f64,
    time_taken: f64,
    res: Vec<(String, bool)>,
}

impl Result {
    pub fn result(&self) {
        println!("");
        println!("{}", "Result".to_uppercase().green());
        println!("");
        for word in &self.res {
            let (typed, correct) = word;
            if !*correct {
                print!("{} ", &typed.red());
            } else {
                print!("{} ", &typed)
            }
        }
        println!("\n");
        println!("WPM : {}", &self.wpm.to_string().green());
        println!("Raw WPM: {}", &self.raw.to_string().green());
        println!(
            "Time Taken : {}",
            (String::from(&self.time_taken.to_string()) + "s").green()
        );
        println!("");
    }
}

pub fn start() -> Info {
    println!("{}", "TYPING TEST".green());
    let args: Vec<_> = env::args().collect();
    let mut iter = args.into_iter();
    iter.next();

    let mut num: String = match iter.next() {
        Some(val) => val,
        None => String::new(),
    };

    if !(num.len() > 0) {
        println!("How many words? ");
        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");
    }
    let count = num.trim().parse().unwrap();
    Info {
        count,
        words: get_words(count),
    }
}

pub fn get_words(num: usize) -> Vec<String> {
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
    println!("{}", "The words are");
    println!("");
    println!("{}", info.words.join(" ").cyan());
    println!("");
    let mut input = String::new();

    println!("Starting in");
    thread::sleep(Duration::from_secs(1));
    for i in 1..4 {
        println!("{}", (4 - i).to_string().green());
        thread::sleep(Duration::from_secs(1));
    }
    println!("{}", "GO".bold());
    println!("");
    let start = std::time::SystemTime::now();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let end = std::time::SystemTime::now();

    return Test {
        input: String::from(input.trim()),
        start,
        end,
    };
}

pub fn get_result(input: Test, info: Info) -> Result {
    let mut res: Vec<(String, bool)> = Vec::new();
    let mut words = input.input.split(" ");
    let mut i = 0;
    let mut correct = 0;
    loop {
        let word = match words.next() {
            Some(val) => val,
            None => break,
        };
        let bool = word == info.words[i];
        if bool {
            correct += 1
        }
        res.push((String::from(word), bool));
        i += 1;
        if i >= info.count {
            break;
        }
    }
    let time_taken = input.end.duration_since(input.start).unwrap().as_secs_f64();
    let mins: f64 = time_taken / 60.0;
    let wpm = (f64::from(correct) / mins).round();
    let i: i32 = (i).to_string().parse().unwrap();
    let raw = (f64::from(i) / mins).round();

    Result {
        wpm,
        raw,
        time_taken,
        res,
    }
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
