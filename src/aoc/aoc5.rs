use std::cmp::{max, min};

use crate::io::read_lines;

const FILE: &str = "data/aoc5.txt";


fn part1() -> usize {
    let mut x_max: usize = 0;
    let mut y_max: usize = 0;

    let mut vec = Vec::new();

    for line in read_lines(FILE).unwrap() {
        let line = line.unwrap();
        let mut split = line.split(" -> ");

        let pos1_str = split.next().unwrap();
        let pos2_str = split.next().unwrap();

        let mut pos1_split = pos1_str.split(",");
        let mut pos2_split = pos2_str.split(",");

        let x1 = pos1_split.next().unwrap().parse::<usize>().unwrap();
        let y1 = pos1_split.next().unwrap().parse::<usize>().unwrap();

        x_max = max(x_max, x1);
        y_max = max(y_max, y1);

        let x2 = pos2_split.next().unwrap().parse::<usize>().unwrap();
        let y2 = pos2_split.next().unwrap().parse::<usize>().unwrap();

        x_max = max(x_max, x2);
        y_max = max(y_max, y2);

        vec.push(((x1, y1), (x2, y2)));
    }

    let mut vec2 = vec![vec![0; y_max + 1]; x_max + 1];

    for ((x1, y1), (x2, y2)) in vec {
        if x1 == x2 {
            for y in min(y1, y2)..max(y1, y2) + 1 {
                vec2[x1][y] += 1
            }
        } else if y1 == y2 {
            for x in min(x1, x2)..max(x1, x2) + 1 {
                vec2[x][y1] += 1
            }
        }
    }

    vec2
        .iter()
        .flat_map(|array| array.iter())
        .filter(|x| **x >= 2)
        .count()
}

fn part2() -> i32 {
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 5147);
    }

    #[test]
    fn test_part2() {
        println!("{}", part2());
    }
}

