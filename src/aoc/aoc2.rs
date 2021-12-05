use std::fmt::Debug;
use crate::io::read_lines;

const FILE: &str = "data/aoc2.txt";

pub fn part1() {
    let mut distance = 0;
    let mut depth = 0;
    for line in read_lines(FILE).unwrap() {
        let (instruction, x) = parse_line(&line);

        match instruction {
            "forward" => distance += x,
            "up" => depth -= x,
            "down" => depth += x,
            _ => panic!("Unrecognised instruction: {}", instruction),
        }
    }
    println!("{}", distance * depth);
}


pub fn part2() {
    let mut distance = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in read_lines(FILE).unwrap() {

        let (instruction, x) = parse_line(&line);

        match instruction {
            "forward" => {
                distance += x;
                depth += aim * x;
            }
            "up" => aim -= x,
            "down" => aim += x,
            _ => panic!("Unrecognised instruction: {}", instruction),
        }
    }
    println!("{}", distance * depth);
}

fn parse_line<E: Debug>(val: &Result<String, E>) -> (&str, i32) {
    let mut split = val.as_ref().unwrap().split(" ");
    let instruction = split.next().unwrap();
    let x = split.next().unwrap().parse::<i32>().unwrap();
    (instruction, x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        part1();
    }

    #[test]
    fn test_part2() {
        part2();
    }
}
