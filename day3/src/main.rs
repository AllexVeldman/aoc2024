use std::{collections::BTreeMap, fs};

use regex::Regex;

fn read_input() -> String {
    fs::read_to_string("input.txt").expect("Could not read input file")
}

/// find and execute "mul(a,b)"
fn puzzle_1(line: &str) -> u32 {
    find_mul(line)
}

fn find_mul(line: &str) -> u32 {
    let re = Regex::new(r"mul\((?<lhs>[0-9]{1,3}),(?<rhs>[0-9]{1,3})\)").expect("valid regex");

    re.captures_iter(line).fold(0, |acc, hit| {
        acc + (hit["lhs"].parse::<u32>().unwrap() * hit["rhs"].parse::<u32>().unwrap())
    })
}

fn puzzle_2(line: &str) -> u32 {
    find_maybe_mul(line)
}

fn find_maybe_mul(line: &str) -> u32 {
    let mul_re = Regex::new(r"mul\((?<lhs>[0-9]{1,3}),(?<rhs>[0-9]{1,3})\)").expect("valid regex");
    let do_re = Regex::new(r"do\(\)").expect("valid regex");
    let dont_re = Regex::new(r"don't\(\)").expect("valid regex");

    // Ordered map of do's and dont's
    let mut map = BTreeMap::new();
    do_re.find_iter(line).for_each(|hit| {
        map.insert(hit.start(), true);
    });
    dont_re.find_iter(line).for_each(|hit| {
        map.insert(hit.start(), false);
    });
    let mut acc = 0;
    for hit in mul_re.captures_iter(line) {
        let location = hit.name("lhs").unwrap().start();

        // find the last do/don't before the location
        let mut enabled = true;
        for (idx, enable) in map.iter() {
            // if idx is > location, we past the hit, the previous loop determines our enable
            if *idx > location {
                break;
            }
            enabled = *enable;
        }
        if enabled {
            acc += hit["lhs"].parse::<u32>().unwrap() * hit["rhs"].parse::<u32>().unwrap();
        }
    }
    acc
}

fn main() {
    let data = read_input();
    println!("{}", puzzle_1(&data));
    println!("{}", puzzle_2(&data));
}

#[cfg(test)]
mod test {
    use crate::{find_maybe_mul, find_mul};

    #[test]
    fn test_puzzle_1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(find_mul(input), 161);
    }
    #[test]
    fn test_puzzle_2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";
        assert_eq!(find_maybe_mul(input), 96);
    }
}
