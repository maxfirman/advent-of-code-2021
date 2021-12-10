use std::collections::{HashMap, HashSet};
use std::str::Split;
use crate::io::read_lines;
use itertools::Itertools;

const FILE: &str = "data/aoc8.txt";

fn part1() -> usize {
    read_lines(FILE)
        .unwrap()
        .map(|line|
            line
                .unwrap()
                .split("|")
                .last()
                .unwrap()
                .split(" ")
                .filter(|x| [2usize, 3usize, 4usize, 7usize].contains(&x.trim().len()))
                .count()
        )
        .sum()
}

fn part2() -> i32 {
    let letters = "abcdefg";
    let letters_list = letters.chars().collect::<Vec<char>>();
    let letters_set = letters.chars().collect::<HashSet<char>>();
    let mut d3: HashMap<_, _> = HashMap::from_iter(letters.chars().enumerate().map(|(a, b)| (b, a)));
    let map: HashMap<_, _> = HashMap::from_iter(
        [
            ("abcefg", "0"),
            ("cf", "1"),
            ("acdeg", "2"),
            ("acdfg", "3"),
            ("bcdf", "4"),
            ("abdfg", "5"),
            ("abdefg", "6"),
            ("acf", "7"),
            ("abcdefg", "8"),
            ("abcdfg", "9"),
        ]
    );

    let d2: HashMap<_, HashSet<_>> = HashMap::from_iter(
        [
            (2, "cf".chars().collect::<HashSet<char>>()),
            (3, "acf".chars().collect::<HashSet<char>>()),
            (4, "bcdf".chars().collect::<HashSet<char>>()),
        ]
    );

    for line in read_lines(FILE).unwrap() {
        let (input, output) = read_data(&line.unwrap());

        println!("{:?}", input);
        println!("{:?}", output);


        let mut valid: HashMap<usize, HashSet<char>> = HashMap::new();

        for word in input {
            let shit = d2.get(&word.len()).unwrap_or(&letters_set);
            for l in word.chars() {
                let k = d3[&l];
                let mut x = valid.get(&k).unwrap_or(&letters_set);
                valid.insert(k, x.intersection(&shit).map(|x| *x).collect::<HashSet<char>>());
            }
        }
        println!("{:?}", valid);
    }
    0
}

fn read_data(line: &str) -> (Vec<String>, Vec<String>) {
    let mut split = line.split(" | ");
    let input = parse(&split.next().unwrap());
    let output = parse(&split.next().unwrap());
    (input, output)
}

fn parse(string: &str) -> Vec<String> {
    string
        .split(" ")
        .map(|x| x.chars().sorted().collect::<String>())
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 362);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1020159);
    }
}

