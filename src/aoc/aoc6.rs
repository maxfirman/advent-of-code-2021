use std::collections::HashMap;

use crate::io::read_lines;

const FILE: &str = "data/aoc6.txt";


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
    let mut nums2 = read_nums();
    let mut nums = nums2.iter().map(|x| *x as usize);

    let mut d1: HashMap<usize, usize> = HashMap::new();
    let mut d2: HashMap<usize, usize> = HashMap::new();

    for i in 0..=7 {
        d1.insert(i, 0);
        d2.insert(i, 0);
    }

    for num in nums {
        *d1.get_mut(&num).unwrap() += 1;
    }

    for i in 0..days {
        let j = i % 7;
        let k = (j + 2) % 7;
        *d2.get_mut(&k).unwrap() += d1[&j];
        *d1.get_mut(&j).unwrap() += d2[&j];
        d2.insert(j, 0);
    }

    return d1.values().sum::<usize>() + d2.values().sum::<usize>();
}

fn read_nums() -> Vec<i32> {
    read_lines(FILE)
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
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
        assert_eq!(part2(256), 1600306001288);
    }
}

