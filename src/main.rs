mod days;

use std::env;
use std::process::exit;
use crate::days::day1::day1;

const DAYS: [fn(&[String]) -> ();1] = [
    day1
];

const NR_OF_DAYS: u32 = DAYS.len() as u32;

fn missing_day(args: &[String]) {
    assert_eq!(args.len(), 1);
    eprintln!("No implementation for day {}", args[0]);
    exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_arg = args
        .get(1)
        .map(|arg: &String| {arg.parse::<u32>()})
        .unwrap_or_else(|| {eprintln!("Missing day argument"); exit(1)})
        .unwrap_or_else(|s| {eprintln!("Cannot parse argument: {}", s); exit(1)});
    (match day_arg {
        1 ..= NR_OF_DAYS => DAYS[day_arg as usize - 1],
        _ => missing_day
    })(&args[2..])
}