use std::env;
use std::fs;


fn day2(content: &String) {
    println!("{:?}", content);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    let day: u8 = day.parse().expect("Not a number");
    let filename = &args[2];
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    match &day {
        1 => {},
        2 => day2(&content),
        _ => {},
    }
}