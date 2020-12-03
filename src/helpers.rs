use std::fs;
use std::path::Path;

pub fn read_from_file<P: AsRef<Path>, T>(fname: P, line_fn : fn(&str) -> T) -> Vec<T> {
    let s = fs::read_to_string(fname).expect("file not found");
    s.lines().map(line_fn).collect()
}
