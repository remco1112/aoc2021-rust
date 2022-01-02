use std::io::{BufRead};
use crate::days::day::Day;

pub struct Day4;

pub struct Input {
    draws: Vec<u32>,
    boards: Vec<[[u32; 5]; 5]>,
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
            boards,
        }
    }

    fn part1(input: &Input) -> u32 {
        let (drawn_numbers, last_draw, board_index) = Self::get_winning_board(input);
        Self::get_score(&input.boards[board_index], &drawn_numbers, last_draw)
    }

    fn part2(input: &Input) -> u32 {
        let (drawn_numbers, last_draw, board_index) = Self::get_last_winning_board(input);
        Self::get_score(&input.boards[board_index], &drawn_numbers, last_draw)
    }
}

impl Day4 {
    fn get_winning_board(input: &Input) -> ([[u32; 5]; 5], u32, usize) {
        let mut boards: Vec<[[u32; 5]; 5]> = vec![Default::default(); input.boards.len()];
        for &draw in &input.draws {
            for board in 0..boards.len() {
                for row in 0..5 {
                    for col in 0..5 {
                        if input.boards[board][row][col] == draw {
                            boards[board][row][col] = 1;
                            if Self::has_won(&boards[board], row, col) {
                                return (boards[board], draw, board);
                            }
                        }
                    }
                }
            }
        }
        panic!("Did not find a winning board!");
    }

    fn get_score(board: &[[u32; 5]; 5], drawn_numbers: &[[u32; 5]; 5], draw: u32) -> u32 {
        let mut sum: u32 = 0;
        for row in 0..5 {
            for col in 0..5 {
                if drawn_numbers[row][col] == 0 {
                    sum += board[row][col];
                }
            }
        }
        sum * draw
    }

    fn get_last_winning_board(input: &Input) -> ([[u32; 5]; 5], u32, usize) {
        let mut boards: Vec<[[u32; 5]; 5]> = vec![Default::default(); input.boards.len()];
        let mut last_board : (u32, usize) = Default::default();
        let mut won_boards: Vec<bool> = vec![false; input.boards.len()];
        for &draw in &input.draws {
            'board: for board in 0..boards.len() {
                if  won_boards[board] {
                    continue;
                }
                for row in 0..5 {
                    for col in 0..5 {
                        if input.boards[board][row][col] == draw {
                            boards[board][row][col] = 1;
                            if Self::has_won(&boards[board], row, col) {
                                last_board = (draw, board);
                                won_boards[board] = true;
                                continue 'board;
                            }
                        }
                    }
                }
            }
        }
        (boards[last_board.1], last_board.0, last_board.1)
    }

    fn has_won(board: &[[u32; 5]; 5], row: usize, col: usize) -> bool {
        let mut has_row = true;
        let mut has_col = true;
        for check in 0..5 {
            if board[check][col] == 0 {
                has_col = false;
            }
            if board[row][check] == 0 {
                has_row = false;
            }
            if !has_row && !has_col {
                break;
            }
        }
        has_row || has_col
    }
}