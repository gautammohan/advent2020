// Worked with Omar on this one
use std::collections::{HashMap, HashSet};
use std::fs;

pub fn day6() {
    let f = fs::read_to_string("inputs/day6.txt").expect("file not found");
    let groups = f.split("\n\n");

    let mut sum = 0;
    for group in groups {
        let mut s = HashSet::new();
        for person in group.lines() {
            person.chars().for_each(|c| {
                s.insert(c);
            });
        }
        sum += s.len();
    }
    println!("day 6 part 1: {}", sum);

    let mut sum = 0;
    for group in f.split("\n\n") {
        let count = group.lines().count();
        let mut m = HashMap::<char, usize>::new();
        for person in group.lines() {
            person.chars().for_each(|c| {
                *m.entry(c).or_insert(0) += 1;
            });
        }
        sum += m.values().filter(|v| **v == count).count();
    }
    println!("day 6 part 2: {}", sum);
}
