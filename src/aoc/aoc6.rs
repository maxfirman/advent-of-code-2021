use crate::io::read_lines;


const FILE: &str = "data/aoc6.txt";


fn part1() -> usize {
    let mut nums = read_nums();

    for _ in 0..80 {
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

// fn part2() -> usize { 0 }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 352195);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(), 16925);
    // }
}

