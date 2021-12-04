use itermore::IterMore;
use crate::utils::io::read_lines;

const FILE: &str = "data/aoc1.txt";

pub fn part1() {
    let sum: i32 = read_lines(FILE)
        .unwrap()
        .map(|val| val.unwrap().parse::<i32>().unwrap())
        .windows()
        .map(|[prev, next]| (next > prev) as i32)
        .sum();

    println!("{}", sum);
}

pub fn part2() {
    let sum: i32 = read_lines(FILE)
        .unwrap()
        .map(|val| val.unwrap().parse::<i32>().unwrap())
        .windows::<3>()
        .map(|[a, b, c]| a + b + c)
        .windows()
        .map(|[prev, next]| (next > prev) as i32)
        .sum();

    println!("{}", sum);
}
