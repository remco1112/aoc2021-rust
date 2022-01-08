use std::cmp::{max, min};
use std::io::{BufRead};
use std::iter::Zip;
use regex::Regex;
use crate::days::day::Day;

pub struct Day5;

type Line = ((usize, usize), (usize, usize));
type Grid = [[usize; 1000]; 1000];

impl Day<Vec<Line>> for Day5 {
    fn parse(reader: &mut dyn BufRead) -> Vec<Line> {
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

    fn part1(input: &Vec<Line>) -> u32 {
        Self::get_solution(input, Self::mark_grid_part1)
    }

    fn part2(input: &Vec<Line>) -> u32 {
        Self::get_solution(input, Self::mark_grid_part2)
    }
}

impl Day5 {
    fn get_grid(input: &Vec<Line>, grid_marker: fn(&Line, &mut Grid)) -> Grid {
        let mut grid = [[0; 1000]; 1000];
        for line in input {
            grid_marker(line, &mut grid)
        }
        grid
    }

    fn get_overlap_count(grid: &Grid) -> u32 {
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

    fn get_solution(input: &Vec<Line>, grid_marker: fn(&Line, &mut Grid)) -> u32 {
        Self::get_overlap_count(&Self::get_grid(input, grid_marker))
    }

    fn mark_grid_part1(line: &Line, grid: &mut Grid) {
        Self::mark_vertical(line, grid);
        Self::mark_horizontal(line, grid);
    }

    fn mark_grid_part2(line: &Line, grid: &mut Grid) {
        Self::mark_vertical(line, grid);
        Self::mark_horizontal(line, grid);
        Self::mark_diagonal(line, grid);
    }

    fn mark_vertical(line: &Line, grid: &mut Grid) {
        let &((x_1, y_1), (x_2, y_2)) = line;
        if x_1 == x_2 {
            for y in min(y_1, y_2)..=max(y_1, y_2) {
                grid[y][x_1] += 1;
            }
        }
    }

    fn mark_horizontal(line: &Line, grid: &mut Grid) {
        let &((x_1, y_1), (x_2, y_2)) = line;
        if y_1 == y_2 {
            for x in min(x_1, x_2)..=max(x_1, x_2) {
                grid[y_1][x] += 1;
            }
        }
    }


    fn mark_diagonal(line: &Line, grid: &mut Grid) {
        let &((x_1, y_1), (x_2, y_2)) = line;
        if i32::abs(y_1 as i32 - y_2 as i32) == i32::abs(x_1 as i32 - x_2 as i32) {
            for (x, y) in Self::get_line_iter(line) {
                grid[y][x] += 1;
            }
        }
    }

    fn get_range_iter(x_1: usize, x_2: usize) -> Box<dyn DoubleEndedIterator<Item=usize>> {
        if x_2 > x_1 { Box::new(x_1..=x_2) } else { Box::new((x_2..=x_1).rev()) }
    }

    fn get_line_iter(line: &Line) -> Zip<Box<dyn DoubleEndedIterator<Item=usize>>, Box<dyn DoubleEndedIterator<Item=usize>>> {
        let &((x_1, y_1), (x_2, y_2)) = line;
        let x_iter: Box<dyn DoubleEndedIterator<Item=usize>> = Self::get_range_iter(x_1, x_2);
        let y_iter: Box<dyn DoubleEndedIterator<Item=usize>> = Self::get_range_iter(y_1, y_2);
        x_iter.zip(y_iter)
    }
}