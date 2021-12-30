use std::io::BufRead;

use crate::days::day::Day;

const WORDSIZE: usize = 12;

pub struct Day3;

impl Day<Vec<u32>> for Day3 {
    fn parse(reader: &mut dyn BufRead) -> Vec<u32> {
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
        let mut bit_cntrs: [u32; WORDSIZE] = Default::default();
        for i in numbers {
            for j in 0..WORDSIZE {
                bit_cntrs[j] += (i & (1 << j)) >> j
            }
        }
        let mut result: u32 = 0;
        for i in 0..WORDSIZE {
            result += ((bit_cntrs[i] > size / 2) as u32) << i
        }
        result * (!result & 0xFFF)
    }

    fn part2(numbers: &Vec<u32>) -> u32 {
        let mut o2_numbers = numbers.clone();
        let mut co2_numbers = numbers.clone();
        let mut o2_done: bool = false;
        let mut co2_done: bool = false;
        for j in (0..WORDSIZE).rev() {
            if !o2_done {
                let mut split_o2_numbers = [Vec::new(), Vec::new()];
                for &i in &o2_numbers {
                    split_o2_numbers[((i & (1 << j)) >> j) as usize].push(i)
                }
                let new_o2_numbers = &split_o2_numbers[(split_o2_numbers[0].len() <= split_o2_numbers[1].len()) as usize];
                o2_numbers.resize(new_o2_numbers.len(), 0);
                o2_numbers.copy_from_slice(new_o2_numbers);
                if o2_numbers.len() == 1 {
                    o2_done = true
                }
            }
            if !co2_done {
                let mut split_co2_numbers: [Vec::<u32>; 2] = [Vec::new(), Vec::new()];
                for &i in &co2_numbers {
                    split_co2_numbers[((i & (1 << j)) >> j) as usize].push(i)
                }
                let new_co2_numbers = &split_co2_numbers[(split_co2_numbers[0].len() > split_co2_numbers[1].len() && !split_co2_numbers[1].is_empty()) as usize];
                co2_numbers.resize(new_co2_numbers.len(), 0);
                co2_numbers.copy_from_slice(new_co2_numbers);
                if co2_numbers.len() == 1 {
                    co2_done = true;
                }
            }
            if o2_done && co2_done {
                break;
            }
        }
        o2_numbers[0] * co2_numbers[0]
    }
}