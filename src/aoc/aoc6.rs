use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use crate::io::read_lines;

const FILE: &str = "data/aoc6.txt";
const N: usize = 7;

fn part1(days: usize) -> usize {
    let mut nums = read_nums();

    for _ in 0..days {
        let n = nums
            .iter()
            .filter(|x| **x == 0)
            .count();

        nums = nums
            .iter()
            .map(|x| if *x > 0 { x - 1 } else { 6 })
            .collect();

        nums.extend([8].repeat(n));
    }

    nums.len()
}

fn part2(days: usize) -> usize {
    let nums = read_nums();

    let mut d1: HashMap<usize, usize> = HashMap::new();
    let mut d2: HashMap<usize, usize> = HashMap::new();

    for num in nums {
        *d1.entry(num).or_insert(0) += 1
    }

    for i in 0..days {
        let j = i % N;
        let k = (j + 2) % N;
        *d2.entry(k).or_insert(0) += d1.get(&j).unwrap_or(&0);
        *d1.entry(j).or_insert(0) += d2.get(&j).unwrap_or(&0);
        d2.insert(j, 0);
    }

    return d1.values().sum::<usize>() + d2.values().sum::<usize>();
}

fn read_nums<T>() -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: fmt::Debug,
{
    read_lines(FILE)
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<T>().unwrap())
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(80), 352195);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part1(80), part2(80));
        assert_eq!(part2(256), 1600306001288);
    }
}

