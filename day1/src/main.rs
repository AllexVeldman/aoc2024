use core::panic;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter::zip,
};

fn read_input() -> (Vec<u32>, Vec<u32>) {
    let file = File::open("input.txt").expect("Could not read file");
    let lines = BufReader::new(file).lines();

    let mut left = vec![];
    let mut right = vec![];
    for line in lines {
        let line = line.unwrap();
        let [left_id, right_id] = line.split_whitespace().collect::<Vec<&str>>()[..] else {
            panic!("invalid input")
        };
        left.push(left_id.parse().expect("Parse string to int"));
        right.push(right_id.parse().expect("Parse string to int"));
    }
    (left, right)
}

/// Distance between sorted lists
fn puzzle_1(left: &mut Vec<u32>, right: &mut Vec<u32>) -> u32 {
    left.sort_unstable();
    right.sort_unstable();
    let result = zip(left, right).fold(0, |acc, (left, right)| acc + left.abs_diff(*right));
    println!("{result}");
    result
}

/// Similarity
fn puzzle_2(left: Vec<u32>, right: Vec<u32>) -> u32 {
    let mut counts = HashMap::new();
    for value in right {
        *counts.entry(value).or_insert(0) += 1;
    }
    let result = left.iter().fold(0, |acc, value| {
        acc + value * counts.get(value).unwrap_or(&0)
    });
    println!("{result}");
    result
}

fn main() {
    let (mut left, mut right) = read_input();
    puzzle_1(&mut left, &mut right);
    puzzle_2(left, right);
}

#[cfg(test)]
mod test {
    #[test]
    fn puzzle_1() {
        let result = crate::puzzle_1(&mut vec![3, 4, 2, 1, 3, 3], &mut vec![4, 3, 5, 3, 9, 3]);
        assert_eq!(result, 11);
    }

    #[test]
    fn puzzle_2() {
        let result = crate::puzzle_2(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]);
        assert_eq!(result, 31);
    }
}
