use crate::io::read_lines;

const FILE: &str = "data/aoc3.txt";
const N: usize = 12;
const RADIX: u32 = 10;

type Bits = [bool; N];


fn part1() -> u32 {
    let mut array = [0; N];
    let mut n = 0;
    for line in read_lines(FILE).unwrap() {
        let line = line.unwrap();
        let mut split = line.chars();
        for i in 0..N {
            array[i] += split.next().unwrap().to_digit(RADIX).unwrap();
        }
        n += 1;
    }

    let gamma = as_integer(array.map(|x| x > n / 2));
    let epsilon = as_integer(array.map(|x| x < n / 2));

    return gamma * epsilon;
}

fn part2() -> u32 {
    let vec = read_file_to_vector();

    let res1 = as_integer(filter(vec.clone(), pick_longest));
    let res2 = as_integer(filter(vec.clone(), pick_shortest));

    return res1 * res2;
}

fn pick_longest<T>(v1: Vec<T>, v2: Vec<T>) -> Vec<T> {
    if v1.len() >= v2.len() { v1 } else { v2 }
}

fn pick_shortest<T>(v1: Vec<T>, v2: Vec<T>) -> Vec<T> {
    if v1.len() >= v2.len() { v2 } else { v1 }
}

fn as_integer(bits: Bits) -> u32 {
    let mut res: u32 = 0;
    for i in 0..N {
        res |= (bits[i] as u32) << (N - i - 1);
    }
    return res;
}

fn read_file_to_vector() -> Vec<Bits> {
    let mut vec = Vec::new();
    let mut n = 0;
    for line in read_lines(FILE).unwrap() {
        let line = line.unwrap();
        let mut chars = line.chars();
        let mut arr = [false; N];
        for i in 0..N {
            arr[i] = chars.next().unwrap().to_digit(RADIX).unwrap() != 0;
        }
        vec.push(arr);
        n += 1;
    }
    vec
}

fn filter(mut v: Vec<Bits>, f: fn(Vec<Bits>, Vec<Bits>) -> Vec<Bits>) -> Bits {
    let mut i = 0;
    while v.len() > 1 {
        let (v1, v2): (Vec<Bits>, Vec<Bits>) = v.iter().partition(|arr| arr[i]);
        v = f(v1, v2);
        i += 1
    }
    return v[0];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 693486);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 3379326);
    }
}
