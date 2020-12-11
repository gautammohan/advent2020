use std::collections::VecDeque;
use std::fs;

type N = i128;

pub fn day9() {
    let f = fs::read_to_string("inputs/day9.txt").expect("file not found");
    let mut nums: Vec<N> = vec![];
    for line in f.lines() {
        nums.push(line.parse().expect("bad number parse"));
    }

    let mut window: VecDeque<_> = VecDeque::with_capacity(25);
    for i in 0..25 {
        window.push_back(nums[i]);
    }

    for num in &nums[25..] {
        if !check_valid(&window, *num) {
            println!("day 9 part 1: {}", num);
            println!("day 9 part 2: {}", part_2(&nums, *num));
            break;
        }
        window.pop_front();
        window.push_back(*num);
    }
}

fn check_valid(window: &VecDeque<N>, m: N) -> bool {
    for n in window {
        // This would be faster if we had some kind of ordered hashset
        // but whatever.
        if window.contains(&(m - n)) {
            return true;
        }
    }
    false
}

fn part_2(nums: &Vec<N>, target: N) -> N {
    for i in 0..nums.len() - 1 {
        let mut j = i + 1;
        let mut sum = nums[i];
        while j < nums.len() && sum < target {
            sum += nums[j];
            if sum == target {
                return nums[i..j].iter().min().unwrap() + nums[i..j].iter().max().unwrap()
            }
            j += 1;
        }
    }
    panic!("not found");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part2() {
        let v: Vec<N> = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(part_2(&v, 127), 62);
    }
}
