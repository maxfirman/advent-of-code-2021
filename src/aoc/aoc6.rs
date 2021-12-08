use crate::io::read_nums;

const FILE: &str = "data/aoc6.txt";
const N: usize = 7;

fn part1(days: usize) -> usize {
    let mut nums: Vec<i32> = read_nums(FILE).collect();

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
    let mut l1 = [0; N];
    let mut l2 = [0; N];

    for num in read_nums::<usize>(FILE) {
        l1[num] += 1
    }

    for i in 0..days {
        let j = i % N;
        let k = (j + 2) % N;
        l2[k] += l1[j];
        l1[j] += l2[j];
        l2[j] = 0;
    }

    l1.iter().sum::<usize>() + l2.iter().sum::<usize>()
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

