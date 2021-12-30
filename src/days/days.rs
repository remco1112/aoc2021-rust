use std::io::BufRead;

use crate::days::day1::Day1;
use crate::days::day2::Day2;
use crate::days::day3::Day3;
use crate::days::day4::Day4;
use crate::days::day::Day;

pub const NR_OF_DAYS: u32 = 4;

pub fn get_result_for_day(reader: &mut dyn BufRead, day: u32) -> (u32, u32) {
    match day {
        1 => Day1.all(reader),
        2 => Day2.all(reader),
        3 => Day3.all(reader),
        4 => Day4.all(reader),
        _ => panic!("No implementation for day {}", day)
    }
}