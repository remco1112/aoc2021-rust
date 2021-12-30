use std::io::{BufRead};
use crate::days::day::Day;

pub struct Day4;

pub struct Input {
    draws: Vec<u32>,
    boards: Vec<[[u32; 5]; 5]>
}

impl Day<Input> for Day4 {
    fn parse(reader: &mut dyn BufRead) -> Input {
        let mut read_line = || -> Option<String> {
            let mut line = String::new();
            let res = reader.read_line(&mut line)
                .expect("Parse error");
            match res {
                0 => None,
                _ => Some(line)
            }
        };
        let draw_line = read_line().expect("Unexpected EOF");
        let draws = draw_line
            .trim()
            .split(",")
            .map(|s| s.parse::<u32>().expect(&*format!("Failed to parse {} as number", s)))
            .collect::<Vec<_>>();
        let mut boards: Vec<[[u32; 5]; 5]> = Vec::new();
        loop {
            if read_line().is_none() {
                break;
            }
            let mut board: [[u32; 5]; 5] = Default::default();
            for i in 0..5 {
                read_line()
                    .expect("Unexpected EOF")
                    .trim()
                    .split_whitespace()
                    .map(|s| s.parse::<u32>().expect(&*format!("Failed to parse {} as number", s)))
                    .enumerate()
                    .for_each(|(j, n)| {
                        assert!(j < 5);
                        board[i][j] = n;
                    })
            }
            boards.push(board)
        }
        Input {
            draws,
            boards
        }
    }

    fn part1(input: &Input) -> u32 {
        todo!()
    }

    fn part2(input: &Input) -> u32 {
        todo!()
    }
}