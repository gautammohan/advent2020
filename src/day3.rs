use std::fs;

pub fn day3() {
    let f = fs::read_to_string("inputs/day3.txt").expect("file not found");
    let rows: Vec<&str> = f.lines().collect();

    let num_trees = solve(1, 3, &rows);
    println!("day 3 part 1: {}", num_trees);
    let tree_prod = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .iter()
        .fold(1, |acc, (v, h)| acc * solve(*v, *h, &rows));
    println!("day 3 part 2: {}", tree_prod);
}

fn solve(vstep: usize, hstep: usize, rows: &Vec<&str>) -> usize {
    let hsteps = (0..rows.len() * hstep).step_by(hstep);
    let vsteps = (0..rows.len()).step_by(vstep);
    let indices = vsteps.zip(hsteps);
    indices.fold(0, |acc, (i, j)| {
        // indexing from a string slice is a bit ugly
        if rows[i].chars().nth(j % rows[i].len()).unwrap() == '#' {
            acc + 1
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::solve;
    static example: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
    #[test]
    fn test_r1_d1() {
        let rows = example.lines().collect();
        assert_eq!(2, solve(1, 1, &rows));
    }

    #[test]
    fn test_r3_d1() {
        let rows = example.lines().collect();
        assert_eq!(7, solve(1, 3, &rows));
    }
    #[test]
    fn test_r5_d1() {
        let rows = example.lines().collect();
        assert_eq!(3, solve(1, 5, &rows));
    }

    #[test]
    fn test_r7_d1() {
        let rows: Vec<&str> = example.lines().collect();
        assert_eq!(4, solve(1, 7, &rows));
    }

    #[test]
    fn test_r1_d2() {
        let rows: Vec<&str> = example.lines().collect();
        assert_eq!(2, solve(2, 1, &rows));
    }

    #[test]
    fn prod() {
        let rows: Vec<&str> = example.lines().collect();
        let tree_prod = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
            .iter()
            .fold(1, |acc, (h, v)| acc * solve(*h, *v, &rows));
        assert_eq!(336, tree_prod);
    }
}
