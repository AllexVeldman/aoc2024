use core::panic;
use std::{fs, iter::zip};

fn read_input() -> (Vec<u32>, Vec<u32>) {
    let text = fs::read_to_string("input.txt").expect("Could not read the file");
    let lines = text.lines();
    let mut left = vec![];
    let mut right = vec![];
    for line in lines {
        let [left_id, right_id] = line.split_whitespace().collect::<Vec<&str>>()[..] else {
            panic!("invalid input")
        };
        left.push(left_id.parse().expect("Parse string to int"));
        right.push(right_id.parse().expect("Parse string to int"));
    }
    left.sort_unstable();
    right.sort_unstable();
    (left, right)
}

fn puzzle_1() {
    let (left, right) = read_input();
    let result = zip(left, right).fold(0, |acc, (left, right)| acc + left.abs_diff(right));
    println!("{result}");
}

fn main() {
    puzzle_1();
}
