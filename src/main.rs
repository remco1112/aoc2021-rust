use std::env;
use std::fs::{File};
use std::io::{BufRead, BufReader, stdin};

use crate::days::days::{get_result_for_day, NR_OF_DAYS};

mod days;

fn main() {
    let args: Vec<String> = env::args().collect();
    let usage = format!("Usage: {} <day (1-{})> [file]", args[0], NR_OF_DAYS - 1);
    let day_arg = args
        .get(1)
        .map(|arg: &String| {arg.parse::<u32>()})
        .expect(&usage)
        .expect(&usage);
    let mut reader: Box<dyn BufRead> = args.get(2)
        .map(|arg: &String| File::open(arg).expect("Could not open file!"))
        .map_or::<Box<_>, _>(Box::new(BufReader::new(stdin())), |file| Box::new(BufReader::new(file)));
    let (part1, part2) = get_result_for_day(&mut reader, day_arg);
    println!("Part One: {}", part1);
    println!("Part Two: {}", part2);
}