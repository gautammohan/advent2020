// Worked with Omar on this one

use std::collections::HashSet;
use std::fs;

pub fn day5() {
    let f = fs::read_to_string("inputs/day5.txt").expect("file not found");
    let dirs: Vec<(Vec<Direction>, Vec<Direction>)> = f
        .lines()
        .map(|line| {
            let seat: &[char] = &line.chars().collect::<Vec<char>>()[0..7];
            let lr: &[char] = &line.chars().collect::<Vec<char>>()[7..];

            let dir = seat
                .iter()
                .map(|c| match c {
                    'F' => Direction::F,
                    'B' => Direction::B,
                    _ => panic!("cannot parse"),
                })
                .collect();

            let seat: Vec<_> = lr
                .iter()
                .map(|c| match c {
                    'L' => Direction::F,
                    'R' => Direction::B,
                    _ => panic!("cannot parse"),
                })
                .collect();

            (dir, seat)
        })
        .collect();

    let seat_ids: Vec<usize> = dirs
        .iter()
        .map(|d| part1(&d.0) as usize * 8 + part1(&d.1) as usize)
        .collect();
    let max = seat_ids.iter().max().unwrap();
    println!("day 5 part 1: {}", max);

    let mut s: HashSet<usize> = HashSet::new();
    seat_ids.iter().for_each(|n| {
        s.insert(*n);
    });
    let mut my_seat = 0;
    for i in 0..=*max {
        if !s.contains(&i) && i > 100 {
            my_seat = i;
        }
    }
    println!("day 5 part 2: {}", my_seat);
}

enum Direction {
    F,
    B,
}

fn part1(v: &[Direction]) -> u8 {
    let s = v
        .iter()
        .map(|e| match e {
            Direction::F => '0',
            Direction::B => '1',
        })
        .collect::<String>();

    return u8::from_str_radix(&s, 2).unwrap();
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use super::Direction::*;

//     #[test]
//     fn t1() {
//         assert_eq!(part1(&[F, B, F, B, B, F, F], 127), 44);
//         assert_eq!(part1(&[B, F, F, F, B, B, F], 127), 70);
//         assert_eq!(part1(&[F, F, F, B, B, B, F], 127), 14);
//         assert_eq!(part1(&[B, B, F, F, B, B, F], 127), 102);

//         assert_eq!(part1(&[B, B, B], 7), 7);
//         assert_eq!(part1(&[B, F, F], 4), 4);
//     }
// }
