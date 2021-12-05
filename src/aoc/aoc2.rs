use std::fmt::Debug;
use crate::io::read_lines;

const FILE: &str = "data/aoc2.txt";

fn part1() -> i32 {
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
    return distance * depth;
}


fn part2() -> i32 {
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
    return distance * depth;
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
        assert_eq!(part1(), 1451208);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1620141160);
    }
}
