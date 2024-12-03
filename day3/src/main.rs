use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn read_input() -> impl Iterator<Item = String> {
    let file = File::open("input.txt").expect("Could not read file");
    let lines = BufReader::new(file).lines();

    lines.map(|line| line.expect("to read a line"))
}

/// find and execute "mul(a,b)"
fn puzzle_1(lines: impl Iterator<Item = String>) -> u32 {
    lines.fold(0, |acc, line| acc + find_mul(&line))
}

fn find_mul(line: &str) -> u32 {
    let re = Regex::new(r"mul\((?<lhs>[0-9]{1,3}),(?<rhs>[0-9]{1,3})\)").expect("valid regex");

    re.captures_iter(line).fold(0, |acc, hit| {
        acc + (hit["lhs"].parse::<u32>().unwrap() * hit["rhs"].parse::<u32>().unwrap())
    })
}

fn main() {
    let data = read_input();
    println!("{}", puzzle_1(data));
}

#[cfg(test)]
mod test {
    use crate::find_mul;

    #[test]
    fn test_puzzle_1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(find_mul(input), 161);
    }
}
