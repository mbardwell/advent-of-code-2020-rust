use std::env;
use std::fs;
use regex::Regex;

fn day1(content: &String) {
    let re = Regex::new(r"(?P<ints>\d+)\n").unwrap();
    for cap in re.captures_iter(content) {
        println!("{},", &cap["ints"]);
    }
}

fn day2(content: &String) {
    let re = Regex::new(
        r"(?P<cmin>\d+)-(?P<cmax>\d+) (?P<c>\w): (?P<input>\w+)\n"
    ).unwrap();
    for cap in re.captures_iter(content) {
        println!("({},{},b'{}',b\"{}\"),",
        &cap["cmin"], &cap["cmax"], &cap["c"], &cap["input"]);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    let day: u8 = day.parse().expect("Not a number");
    let filename = &args[2];
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    match &day {
        1 => day1(&content),
        2 => day2(&content),
        _ => {},
    }
}