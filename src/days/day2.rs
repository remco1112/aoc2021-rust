use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::days::day2::Movement::{Down, Forward, Up};

enum Movement {
    Up(i32),
    Down(i32),
    Forward(i32),
}

pub fn day2(args: &[String]) -> () {
    assert_eq!(args.len(), 1);
    let f = File::open(&args[0]).expect(&*format!("Could not read file {}", &args[0]));
    let mut reader = BufReader::new(f);
    let data = parse(&mut reader);
    println!("Part One: {}", part1(&data));
    println!("Part Two: {}", part2(&data));
}

fn parse(reader: &mut BufReader<File>) -> Vec<Movement> {
    let mut result: Vec<Movement> = Vec::new();
    let mut line_ctr: u32 = 1;
    loop {
        let mut line: String = String::with_capacity(10);
        match reader.read_line(&mut line).expect(&*format!("Could not read line {}!", line_ctr)) {
            0 => break,
            _ => result.push(
                line
                    .split_once(" ")
                    .and_then(
                        |(direction, amount)| amount
                            .trim()
                            .parse::<i32>()
                            .map_or(None, |num| match direction {
                                "forward" => Some(Forward(num)),
                                "up" => Some(Up(num)),
                                "down" => Some(Down(num)),
                                _ => None
                            })
                    )
                    .expect(&*format!("Could not parse line {}!", line_ctr))
            )
        }
        line_ctr += 1;
    }
    result
}

fn part1(movements: &Vec<Movement>) -> i32 {
    let (depth, distance) = movements.iter().fold((0, 0), |(depth, distance), movement| match movement {
        Up(amount) => (depth - amount, distance),
        Down(amount) => (depth + amount, distance),
        Forward(amount) => (depth, distance + amount)
    });
    depth * distance
}

fn part2(movements: &Vec<Movement>) -> i32 {
    let (depth, distance, _) = movements.iter().fold((0, 0, 0), |(depth, distance, aim), movement|
        match movement {
        Up(amount) => (depth, distance, aim - amount),
        Down(amount) => (depth, distance, aim + amount),
        Forward(amount) => (depth + aim * amount, distance + amount, aim)
    });
    depth * distance
}