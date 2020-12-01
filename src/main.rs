use std::fs;
use std::path::Path;

mod day1;
use day1::*;

fn main() {
    let nums = read_from_file("inputs/day1.txt");
    println!("day 1 part 1: {}", d1p1(&nums));
    println!("day 1 part 2: {}", d1p2(&nums));
}

fn read_from_file<P: AsRef<Path>>(fname: P) -> Vec<i32> {
    let s = fs::read_to_string(fname).expect("file not found");
    let x = s.lines().map(|l| l.parse::<i32>().expect("not an integer")).collect();
    x
}

