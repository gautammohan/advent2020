use std::boxed::Box;
use std::fs;
use std::io::Write;
use std::iter::repeat_with;

extern crate itertools;
use itertools::*;

type Board = Vec<Vec<char>>;

static S: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

pub fn day11() {
    let f = fs::read_to_string("inputs/day11.txt").expect("file not found");
    let mut layout: Board = vec![];
    for line in f.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c);
        }
        layout.push(row);
    }

    println!("day 11 part 1: {}", part_1(&layout));
    println!("day 11 part 2: {}", part_2(&layout));
}

fn _print(b: &Board) {
    for l in b {
        for c in l {
            print!("{}", c)
        }
        println!("");
    }
}

fn part_1(b: &Board) -> i32 {
    let mut new: Board = b.clone();
    let mut old = b.clone();
    let mut count = 0;
    let num_seats = loop {
        let mut changed = false;
        for i in 0..b.len() {
            for j in 0..b[0].len() {
                if old[i][j] == '#' {
                    count += 1;
                }
                let num_occ = num_occupied(&old, i as i32, j as i32);
                let c = old[i][j];
                if num_occ >= 4 && c == '#' {
                    new[i][j] = 'L';
                    changed = true;
                } else if num_occ == 0 && c == 'L' {
                    new[i][j] = '#';
                    changed = true;
                }
            }
        }
        if !changed {
            break count;
        }
        old = new.clone();
        count = 0;
    };
    num_seats
}

fn part_2(b: &Board) -> i32 {
    // all 8 directions. Note: this could be static if I used the
    // lazy_static crate I think. And maybe one day it will be with
    // better const exprs?
    let cardinal_indexers: [Box<dyn Fn((i32, i32)) -> (i32, i32)>; 8] = [
        Box::new(|(x, y)| (x + 1, y)),
        Box::new(|(x, y)| (x + 1, y + 1)),
        Box::new(|(x, y)| (x, y + 1)),
        Box::new(|(x, y)| (x - 1, y + 1)),
        Box::new(|(x, y)| (x - 1, y)),
        Box::new(|(x, y)| (x - 1, y - 1)),
        Box::new(|(x, y)| (x, y - 1)),
        Box::new(|(x, y)| (x + 1, y - 1)),
    ];
    let mut new: Board = b.clone();
    let mut old = b.clone();
    let mut count = 0;
    let num_seats = loop {
        let mut changed = false;
        for i in 0..b.len() {
            for j in 0..b[0].len() {
                if old[i][j] == '#' {
                    count += 1;
                }
                // For each of the 8 directions, generate rays and
                // check whether or not a seat is visible. Count the
                // occupied ones
                let num_occ = cardinal_indexers
                    .iter()
                    .filter(|f| sees_occupied_seat(make_ray(&old, (i as i32, j as i32), *f)))
                    .count();
                let c = old[i][j];
                if num_occ >= 5 && c == '#' {
                    new[i][j] = 'L';
                    changed = true;
                } else if num_occ == 0 && c == 'L' {
                    new[i][j] = '#';
                    changed = true;
                }
            }
        }
        if count == b.len() * b[0].len() {
            panic!("something is wrong")
        }
        if !changed {
            break count;
        }
        old = new.clone();
        count = 0;
    };
    num_seats as i32
}

// a ray takes an indexer function and applies it infinitely
// generating a list of coordinates in one direction. However we make
// sure to clip it to only the bounds of the board
fn make_ray<'a>(
    b: &'a Board,
    coord: (i32, i32),
    indexer: impl Fn((i32, i32)) -> (i32, i32) + 'a,
) -> impl Iterator<Item = char> + 'a {
    //don't want to start at the coord itself
    let mut curr = indexer(coord);
    repeat_with(move || {
        // println!("{:?}", curr);
        let tmp = curr;
        curr = indexer(curr);
        tmp
    })
    .take_while(move |c| check_bounds(b, c.0, c.1))
    .map(move |(x, y)| b[x as usize][y as usize])
}

fn sees_occupied_seat(ray: impl Iterator<Item = char>) -> bool {
    for c in ray {
        match c {
            'L' => return false,
            '#' => return true,
            _ => (),
        }
    }
    false
}

fn check_bounds(b: &Board, r: i32, c: i32) -> bool {
    let max_row = b.len() as i32;
    let max_col = b[0].len() as i32;
    r >= 0 && c >= 0 && r < max_row && c < max_col
}

fn num_occupied(b: &Board, x: i32, y: i32) -> i32 {
    let range: Vec<i32> = vec![-1, 0, 1];
    let idxs = iproduct!(range.iter(), range.iter())
        .filter(|&p| p != (&0, &0))
        .filter(|&(&i, &j)| {
            let r = x as i32 + i;
            let c = y as i32 + j;
            check_bounds(b, r, c)
        });
    idxs.filter(|&(i, j)| b[(x + *i) as usize][(y + *j) as usize] == '#')
        .count() as i32
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1() {
        let mut layout: Board = vec![];
        for line in S.lines() {
            let mut row = vec![];
            for c in line.chars() {
                row.push(c);
            }
            layout.push(row);
        }
        assert_eq!(part_1(&layout), 37);
    }

    #[test]
    fn part2() {
        let mut layout: Board = vec![];
        for line in S.lines() {
            let mut row = vec![];
            for c in line.chars() {
                row.push(c);
            }
            layout.push(row);
        }
        println!("hi");
        assert_eq!(part_2(&layout), 26);
    }
}
