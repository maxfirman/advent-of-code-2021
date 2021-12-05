use std::collections::HashSet;

use regex;

use crate::io::read_lines;

const FILE: &str = "data/aoc4.txt";
const N: usize = 5;

type Board = [[i32; N]; N];

fn part1() -> i32 {
    let (numbers, boards) = read_file();

    let mut numbers_set = HashSet::new();

    for number in numbers {
        numbers_set.insert(number);
        for board in &boards {
            if check_board(&board, &numbers_set) {
                return compute_score(board, &numbers_set, number);
            }
        }
    }
    panic!("No winners found!")
}

fn part2() -> i32 {
    let (numbers, boards) = read_file();

    let mut numbers_set = HashSet::new();

    let mut board_status = vec![false; boards.len()];

    for number in numbers {
        numbers_set.insert(number);
        for (i, board) in boards.iter().enumerate() {
            board_status[i] = check_board(&board, &numbers_set);
            if board_status.iter().all(|x| *x) {
                return compute_score(board, &numbers_set, number);
            }
        }
    }

    panic!("No winners found!")
}

fn compute_score(board: &Board, numbers_set: &HashSet<i32>, number: i32) -> i32 {
    let sum: i32 = board
        .iter()
        .flat_map(|array| array.iter())
        .filter(|x| !numbers_set.contains(x))
        .sum();
    return sum * number;
}

fn regular(i: usize, j: usize, board: &Board) -> i32 {
    board[i][j]
}

fn transpose(i: usize, j: usize, board: &Board) -> i32 {
    board[j][i]
}

fn check_rows(board: &Board, nums: &HashSet<i32>, get: fn(usize, usize, &Board) -> i32) -> bool {
    (0..N)
        .map(|i| (0..N)
            .map(|j| get(i, j, &board))
            .all(|x| nums.contains(&x)))
        .any(|x| x)
}

fn check_board(board: &Board, nums: &HashSet<i32>) -> bool {
    check_rows(&board, &nums, regular) | check_rows(&board, &nums, transpose)
}

fn read_file() -> (Vec<i32>, Vec<Board>) {
    let mut lines = read_lines(FILE).unwrap();
    let numbers: Vec<i32> = lines.next().unwrap().unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    let re = regex::Regex::new(" +").unwrap();
    let mut boards = Vec::new();

    while let Some(_) = lines.next() {
        let mut board = [[0; N]; N];
        for i in 0..N {
            let line = lines.next().unwrap().unwrap();
            let mut split = re.split(&line.trim());
            for j in 0..N {
                board[i][j] = split.next().unwrap().parse::<i32>().unwrap();
            }
        }
        boards.push(board);
    }
    (numbers, boards)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 65325);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 4624);
    }
}

