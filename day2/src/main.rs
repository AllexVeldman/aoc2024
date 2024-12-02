use std::{fs::File, io::BufRead, io::BufReader};

/// input is lines of numbers
fn read_input() -> Vec<Vec<u32>> {
    let file = File::open("input.txt").expect("Could not read file");
    let lines = BufReader::new(file).lines();

    let mut data = vec![];
    for line in lines {
        let line = line.unwrap();
        let line = line
            .split_whitespace()
            .map(|value| value.parse::<u32>().unwrap())
            .collect();
        data.push(line);
    }
    data
}

/// Find safe lines
fn puzzle_1(data: Vec<Vec<u32>>) -> u32 {
    data.iter().fold(0, |acc, line| acc + safe(line))
}

enum Dir {
    Inc,
    Dec,
}

/// line is safe if:
///     - values are all increasing or decreasing
///     - adjecent values are at least 1, at most 3 apart
fn safe(line: &[u32]) -> u32 {
    assert!(line.len() > 2);
    let direction = match (line[0], line[1]) {
        (a, b) if a < b => Dir::Inc,
        (a, b) if a > b => Dir::Dec,
        _ => return 0,
    };
    for pair in line.windows(2) {
        let [a, b] = pair else {
            panic!("no pair found")
        };
        match a.abs_diff(*b) {
            1..=3 => (),
            _ => return 0,
        }
        match direction {
            Dir::Inc if a > b => return 0,
            Dir::Dec if a < b => return 0,
            _ => (),
        }
    }
    1
}

fn main() {
    let input = read_input();
    println!("{}", puzzle_1(input));
}

#[cfg(test)]
mod test {
    #[test]
    fn test_puzzle_1() {
        let input = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert_eq!(crate::puzzle_1(input), 2);
    }
}
