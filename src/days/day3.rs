use std::fs::File;
use std::io::{BufRead, BufReader};

const BITSIZE: usize = 12;

pub fn day3(args: &[String]) -> () {
    assert_eq!(args.len(), 1);
    let f = File::open(&args[0]).expect(&*format!("Could not read file {}", &args[0]));
    let mut reader = BufReader::new(f);
    let data = parse(&mut reader);
    println!("Part One: {}", part1(&data));
}

fn parse(reader: &mut BufReader<File>) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    let mut line_ctr: u32 = 1;
    loop {
        let mut line: String = String::with_capacity(13);
        match reader.read_line(&mut line).expect(&*format!("Could not read line {}!", line_ctr)) {
            0 => break,
            _ => result.push(u32::from_str_radix(line.trim(), 2).expect(&*format!("Could not parse number on line {}", line_ctr)))
        }
        line_ctr += 1;
    }
    result
}

fn part1(numbers: &Vec<u32>) -> u32 {
    let size: u32 = numbers.len().try_into().unwrap();
    let mut bit_cntrs: [u32; BITSIZE] = Default::default();
    for i in numbers {
        for j in 0..BITSIZE {
            bit_cntrs[j] += (i & (1 << j)) >> j
        }
    }
    let mut result: u32 = 0;
    for i in 0..BITSIZE {
        result += ((bit_cntrs[i] > size / 2) as u32) << i
    }
    result * (!result & 0xFFF)
}