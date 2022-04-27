mod challenges;

use std::env;

fn main() {
    println!("Welcome to the advent of code 2021 !");
    let day = parse_args();
    match day {
        1 => challenges::day1::run(),
        _ => println!("Not yet implemented...")
    }
}

fn parse_args() -> i32 {
    let err = "You must provide the day argument between >= 1 <= 25. Example: ./advent-of-code-2021 1";
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("{}", err)
    }
    let day = parse_day_arg(&args[1]);

    if day < 1 || day > 25 {
        panic!("{}", err);
    }

    day
}

fn parse_day_arg(day_str: &String) -> i32 {
    match day_str.parse::<i32>() {
        Ok(num) => num,
        Err(_) => -1
    }
}
