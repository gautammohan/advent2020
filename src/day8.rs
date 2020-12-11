use std::collections::HashSet;
use std::fs;

pub fn day8() {
    let f = fs::read_to_string("inputs/day8.txt").expect("file not found");

    let prog = parse(&f).unwrap();
    match run(&prog) {
        Err(InfiniteLoop(answer)) => println!("day 8 part 1: {}", answer),
        _ => println!("cannot be correct"),
    }
    match brute_force(&prog) {
        Ok(acc)  => println!("day 8 part 2: {}",acc),
        _ => println!("cannot be correct")
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Instruction {
    Nop,
    Acc(i32),
    Jmp(i32),
}
use Instruction::*;

fn is_jmp(i : Instruction) -> bool {
    match i {
        Jmp(_) => true,
        _ => false
    }
}

#[derive(Debug, PartialEq)]
enum Error {
    OutOfBounds,
    InfiniteLoop(i32),
    BadSource,
}
use Error::*;

type Program = Vec<Instruction>;

fn parse(s: &str) -> Result<Program, Error> {
    let mut v: Program = vec![];
    for line in s.lines() {
        let raw = line.split(" ").collect::<Vec<_>>();
        match (raw[0], raw[1]) {
            ("nop", _) => v.push(Nop),
            ("acc", numstr) => v.push(Acc(numstr.parse().unwrap())),
            ("jmp", numstr) => v.push(Jmp(numstr.parse().unwrap())),
            _ => return Err(BadSource),
        }
    }
    Ok(v)
}

fn run(prog: &Program) -> Result<i32, Error> {
    let mut acc: i32 = 0;
    let mut pc: i32 = 0;
    let mut seen: HashSet<i32> = HashSet::new();
    let result = loop {
        if seen.contains(&pc) {
            break Err(InfiniteLoop(acc));
        }
        seen.insert(pc);
        match prog[pc as usize] {
            Acc(i) => acc += i,
            Jmp(i) => {
                if pc + i < 0 {
                    break Err(OutOfBounds);
                } else if pc + i >= prog.len() as i32 {
                    break Ok(acc);
                } else {
                    pc += i;
                    continue;
                }
            }
            Nop => (),
        }
        if pc == prog.len() as i32 {
            break Ok(acc);
        }
        pc += 1;
    };
    result
}

fn brute_force(prog: &Program) -> Result<i32, Error> {
    let jmplocs = (0..prog.len()).filter(|i| is_jmp(prog[*i])).collect::<Vec<_>>();
    for loc in jmplocs {
        let mut new_prog = (*prog).clone(); // :(
        new_prog[loc] = Nop;
        if let Ok(acc) = run(&new_prog) {
            return Ok(acc)
        }
    }
    panic!("could not find a valid jmp replacement")
}

#[cfg(test)]
mod test {
    use super::*;

    static S: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn try_parse() {
        assert!(parse(S).is_ok())
    }
    #[test]
    fn example() {
        let prog = parse(S).unwrap();
        assert_eq!(Err(InfiniteLoop(5)), run(&prog))
    }
}
