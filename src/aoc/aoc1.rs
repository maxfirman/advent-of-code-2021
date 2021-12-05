use itermore::IterMore;
use crate::io::read_lines;

const FILE: &str = "data/aoc1.txt";

fn part1() -> i32 {
    read_lines(FILE)
        .unwrap()
        .map(|val| val.unwrap().parse::<i32>().unwrap())
        .windows()
        .map(|[prev, next]| (next > prev) as i32)
        .sum()
}

fn part2() -> i32 {
    read_lines(FILE)
        .unwrap()
        .map(|val| val.unwrap().parse::<i32>().unwrap())
        .windows::<3>()
        .map(|x| -> i32 { x.iter().sum() })
        .windows()
        .map(|[prev, next]| (next > prev) as i32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1681);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1704);
    }
}

