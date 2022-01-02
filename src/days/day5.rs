use std::cmp::{max, min};
use std::io::{BufRead};
use regex::Regex;
use crate::days::day::Day;

pub struct Day5;

impl Day<Vec<((usize, usize), (usize, usize))>> for Day5 {
    fn parse(reader: &mut dyn BufRead) -> Vec<((usize, usize), (usize, usize))> {
        let regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)\n?$").unwrap();
        let mut line = String::with_capacity(11);
        let mut lines: Vec<((usize, usize), (usize, usize))> = Default::default();
        while reader.read_line(&mut line).unwrap() > 0 {
            if let [x_1, y_1, x_2, y_2] = regex.captures(&line).unwrap()
                .iter()
                .skip(1)
                .map(|n| n.unwrap().as_str().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()[..] {
                lines.push(((x_1, y_1), (x_2, y_2)));
                line.clear();
                continue;
            }
            panic!()
        }
        lines
    }

    fn part1(input: &Vec<((usize, usize), (usize, usize))>) -> u32 {
        Self::get_overlap_count(&Self::get_grid(input))
    }

    fn part2(input: &Vec<((usize, usize), (usize, usize))>) -> u32 {
        todo!()
    }
}

impl Day5 {
    fn get_grid(input: &Vec<((usize, usize), (usize, usize))>) -> [[usize; 1000]; 1000] {
        let mut grid = [[0; 1000]; 1000];
        for &line in input {
            let ((x_1, y_1), (x_2, y_2)) = line;
            if x_1 == x_2 {
                for y in min(y_1, y_2)..=max(y_1, y_2) {
                    grid[y][x_1] += 1;
                }
            }
            if y_1 == y_2 {
                for x in min(x_1, x_2)..=max(x_1, x_2) {
                    grid[y_1][x] += 1;
                }
            }
        }
        grid
    }

    fn get_overlap_count(grid: &[[usize; 1000]; 1000]) -> u32 {
        let mut n: u32 = 0;
        for row in grid {
            for &col in row {
                if col > 1 {
                    n += 1
                }
            }
        }
        n
    }
}