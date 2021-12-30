use std::io::BufRead;

pub trait Day<T> {
    fn parse(reader: &mut dyn BufRead) -> T;

    fn part1(input: &T) -> u32;

    fn part2(input: &T) -> u32;

    fn all(&self, reader: &mut dyn BufRead) -> (u32, u32) {
        let input = Self::parse(reader);
        (Self::part1(&input), Self::part2(&input))
    }
}