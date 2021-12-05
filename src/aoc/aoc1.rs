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
    const N: usize = 3;
    let sum: i32 = read_lines(FILE)
        .unwrap()
        .map(|val| val.unwrap().parse::<i32>().unwrap())
        .windows::<N>()
        .map(|x| -> i32 {x.iter().sum()})
        .windows()
        .map(|[prev, next]| (next > prev) as i32)
        .sum();

    println!("{}", sum);
}
