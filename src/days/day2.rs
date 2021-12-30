use std::io::BufRead;

use crate::days::day2::Movement::{Down, Forward, Up};
use crate::days::day::Day;

pub struct Day2;

pub enum Movement {
    Up(i32),
    Down(i32),
    Forward(i32),
}

impl Day<Vec<Movement>> for Day2 {
    fn parse(reader: &mut dyn BufRead) -> Vec<Movement> {
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

    fn part1(movements: &Vec<Movement>) -> u32 {
        let (depth, distance) = movements.iter().fold((0, 0), |(depth, distance), movement| match movement {
            Up(amount) => (depth - amount, distance),
            Down(amount) => (depth + amount, distance),
            Forward(amount) => (depth, distance + amount)
        });
        (depth * distance).try_into().unwrap()
    }

    fn part2(movements: &Vec<Movement>) -> u32 {
        let (depth, distance, _) = movements.iter().fold((0, 0, 0), |(depth, distance, aim), movement|
            match movement {
                Up(amount) => (depth, distance, aim - amount),
                Down(amount) => (depth, distance, aim + amount),
                Forward(amount) => (depth + aim * amount, distance + amount, aim)
            });
        (depth * distance).try_into().unwrap()
    }
}