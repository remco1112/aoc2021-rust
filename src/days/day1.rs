use std::fs::File;
use std::io::{BufReader};
use std::io::BufRead;

pub fn day1(args: &[String]) -> () {
    assert_eq!(args.len(), 1);
    let f = File::open(&args[0]).expect(&*format!("Could not read file {}", &args[0]));
    let mut reader = BufReader::new(f);
    let numbers = parse(&mut reader);
    println!("Part One: {}", part1(&numbers));
    println!("Part Two: {}", part2(&numbers));

}

fn parse(reader: &mut BufReader<File>) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    let mut line_ctr: u32 = 1;
    loop {
        let mut line: String = String::new();
        match reader.read_line(&mut line).expect(&*format!("Could not read line {}!", line_ctr)) {
            0 => break,
            _ => result.push(line
                .trim()
                .parse::<u32>()
                .expect(&*format!("Could not parse number on line {}!", line_ctr)))
        }
        line_ctr += 1;
    }
    result
}

fn part1(numbers: &Vec<u32>) -> u32 {
    let mut first: bool = true;
    let mut prev: u32 = 0;
    let mut result: u32 = 0;
    for i in numbers {
        if !first && *i > prev {
            result += 1;
        }
        prev = *i;
        first = false
    }
    result
}

fn part2(numbers: &Vec<u32>) -> u32 {
    match numbers.len() {
        0..=3 => 0,
        _ => {
            let mut prev: usize = 0;
            let mut prev_sum: u32 = numbers[0..3].iter().sum();
            let mut result: u32 = 0;
            for i in numbers[3..].iter() {
                let new_sum = prev_sum - numbers[prev] + i;
                if new_sum > prev_sum {
                    result += 1
                }
                prev_sum = new_sum;
                prev += 1;
            }
            result
        }
    }
}