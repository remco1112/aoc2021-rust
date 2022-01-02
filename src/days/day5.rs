use std::io::{BufRead};
use regex::Regex;
use crate::days::day::Day;

pub struct Day5;

impl Day<Vec<((u32, u32), (u32, u32))>> for Day5 {
    fn parse(reader: &mut dyn BufRead) -> Vec<((u32, u32), (u32, u32))> {
        let regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)\n?$").unwrap();
        let mut line = String::with_capacity(11);
        let mut lines: Vec<((u32, u32), (u32, u32))> = Default::default();
        while reader.read_line(&mut line).unwrap() > 0 {
            if let [x_1, y_1, x_2, y_2] = regex.captures(&line).unwrap()
                .iter()
                .skip(1)
                .map(|n| n.unwrap().as_str().parse::<u32>().unwrap())
                .collect::<Vec<u32>>()[..] {
                lines.push(((x_1, y_1), (x_2, y_2)));
                line.clear();
                continue;
            }
            panic!()
        }
        lines
    }

    fn part1(input: &Vec<((u32, u32), (u32, u32))>) -> u32 {
        todo!()
    }

    fn part2(input: &Vec<((u32, u32), (u32, u32))>) -> u32 {
        todo!()
    }
}