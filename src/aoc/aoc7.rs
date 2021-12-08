use crate::io::read_nums;

const FILE: &str = "data/aoc7.txt";

fn part1() -> i32 {
    let mut nums: Vec<i32> = read_nums(FILE).collect();

    let mid = nums.len() / 2;
    let med = *nums.select_nth_unstable(mid).1;

    nums.iter().map(|n| (n - med).abs()).sum::<i32>()
}

fn part2() -> i32 {
    let nums: Vec<i32> = read_nums(FILE).collect();
    let mean = nums.iter().sum::<i32>() / nums.len() as i32;

    (mean..=mean + 1)
        .map(|t| {
            nums.iter()
                .map(|n| {
                    let d = (n - t).abs();
                    d * (d + 1) / 2
                })
                .sum::<i32>()
        })
        .min()
        .unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 344605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 93699985);
    }
}

