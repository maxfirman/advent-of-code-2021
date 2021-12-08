use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_split<P>(filename: P, byte: u8) -> io::Result<io::Split<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).split(byte))
}

pub fn read_nums<T>(filename: &str) -> impl Iterator<Item=T>
    where
        T: FromStr,
        <T as FromStr>::Err: fmt::Debug,
{
    read_split(filename, b","[0])
        .unwrap()
        .map(|x| String::from_utf8(x.unwrap()).unwrap().parse::<T>().unwrap())
}