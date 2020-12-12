use std::collections::HashMap;
use std::fs;

pub fn day10() {
    let f = fs::read_to_string("inputs/day10.txt").expect("file not found");
    let mut adapters: Vec<i32> = vec![0];
    for l in f.lines() {
        let n = l.parse().expect("bad number parse");
        adapters.push(n);
    }
    adapters.sort();
    let (diff1, diff3) = part_1(&adapters);
    println!("day 10 part 1: {}", diff1 * diff3);
    println!("day 10 part 2: {}", part_2(&adapters));
}

fn part_1(adapters: &Vec<i32>) -> (i32, i32) {
    let pairs = adapters.iter().zip(adapters[1..].iter());
    let mut diff1 = 0;
    let mut diff3 = 0;
    for (&a1, &a2) in pairs {
        if a2 - a1 == 1 {
            diff1 += 1;
        }
        if a2 - a1 == 3 {
            diff3 += 1;
        }
    }
    (diff1, diff3 + 1)
}

fn part_2(adapters: &Vec<i32>) -> i64 {
    let mut dp: HashMap<i32, i64> = HashMap::with_capacity(adapters.len());
    dp.insert(*adapters.iter().max().unwrap(), 1);
    for &adapter in adapters.iter().rev() {
        // dp.insert(adapter, 1);
        for reachable_adapter in adapter+1..=adapter+3 {
            if let Some(&n) = dp.get(&reachable_adapter) {
                *dp.entry(adapter).or_insert(0) += n;
            }
        }
    }
    *dp.get(&0).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let mut a = vec![0, 16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        a.sort();
        let (diff1, diff3) = part_1(&a);
        assert_eq!(diff1 * diff3, 35);
    }

    #[test]
    fn part2() {
        let mut a = vec![0, 16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        a.sort();
        assert_eq!(part_2(&a), 8);
    }
}
