use std::collections::HashMap;

use std::fs;

pub fn day2() {

    let s = fs::read_to_string("inputs/day2.txt").expect("file not found");
    let items : Vec<_> = s.lines().map(parse_line).collect();
    println!("day 1 part 1: {}", get_all_valid_passwords(items));
    let items2 : Vec<_> = s.lines().map(parse_line2).collect();
    println!("day 1 part 1: {}", get_all_valid_passwords(items2));

}

fn parse_line(line: &str) -> (Policy, Password) {
    let v = line.split(":").collect::<Vec<_>>();
    // yuck
    let policy_part: Vec<_> = v[0].split(" ").collect();
    let nums: Vec<_> = policy_part[0].split("-").collect();

    let p = Policy{letter : policy_part[1].parse().expect("bad char"), min : nums[0].parse().expect("bad min int"), max : nums[1].parse().expect("bad max int")};
    return (p, Password(v[1].trim()));
}

fn parse_line2(line: &str) -> (Policy2, Password) {
    let v = line.split(":").collect::<Vec<_>>();
    // yuck
    let policy_part: Vec<_> = v[0].split(" ").collect();
    let nums: Vec<_> = policy_part[0].split("-").collect();

    let p = Policy2{letter : policy_part[1].parse().expect("bad char"), lpos : nums[0].parse().expect("bad lpos"), rpos : nums[1].parse().expect("bad rpos")};
    return (p, Password(v[1].trim()));
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Policy {
    letter : char,
    min : i32,
    max : i32
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Policy2 {
    letter : char,
    lpos : usize,
    rpos : usize
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Password<'a>(&'a str);

trait PasswordChecker {
    fn check(self, password: Password) -> bool;
}

impl PasswordChecker for Policy {
    fn check(self, password: Password) -> bool {
        let mut m = HashMap::new();
        password.0.chars().for_each(|c| *m.entry(c).or_insert(0) += 1);
        m.get(&self.letter).map_or_else(|| false,|&n| self.min <= n && n <= self.max)
    }
}

impl PasswordChecker for Policy2 {
    fn check(self, password: Password) -> bool {
        let letters = (password.0.chars().nth(self.lpos-1), password.0.chars().nth(self.rpos-1));
        match letters {
            (Some(l), Some(r)) => (l == self.letter && r != l) || (r == self.letter && l != r),
            _ => false
        }
    }
}

fn get_all_valid_passwords<T : PasswordChecker + Copy>(items: Vec<(T, Password)>) -> usize {
    items.iter().fold(0, |acc, (policy, password)| if policy.check(*password) {acc + 1} else {acc})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_password_policy_pair() {
        let s = "1-3 a: abcde";
        assert_eq!(parse_line(s), (Policy{letter: 'a',min : 1, max: 3}, Password("abcde")));
    }

    #[test]
    fn policy_1_works() {
        let s = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        let v:Vec<_> = s.lines().map(parse_line).collect();
        let valid_passwords = get_all_valid_passwords(v);
        assert_eq!(valid_passwords,2);
    }
    #[test]
    fn policy_2_works() {
        let s = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        let v:Vec<_> = s.lines().map(parse_line2).collect();
        let valid_passwords = get_all_valid_passwords(v);
        assert_eq!(valid_passwords,1);
    }
}
