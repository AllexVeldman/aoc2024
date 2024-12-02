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
fn puzzle_1(data: &[Vec<u32>]) -> u32 {
    data.iter().fold(0, |acc, line| acc + is_safe(line) as u32)
}
fn puzzle_2(data: &[Vec<u32>]) -> u32 {
    data.iter()
        .fold(0, |acc, line| acc + maybe_safe(line) as u32)
}

enum Dir {
    Inc,
    Dec,
}

/// line is safe if:
///     - values are all increasing or decreasing
///     - adjacent values are at least 1, at most 3 apart
///
/// Works by retrying with first dropping the left, then the right value
/// This one does not work for 7,5,6,7,8 since break_idx-1 should have been deleted
fn safe(line: &[u32], retry: bool) -> u32 {
    assert!(line.len() >= 2);

    let mut result: u32 = 1;
    let mut left_break_idx = 0;

    let direction = match (line[0], line[1]) {
        (left, right) if left < right => Dir::Inc,
        (left, right) if left > right => Dir::Dec,
        _ if retry => return safe(&line[1..], false),
        _ => return 0,
    };
    for (left_idx, pair) in line.windows(2).enumerate() {
        let [left, right] = pair else {
            panic!("no pair found")
        };
        match left.abs_diff(*right) {
            1..=3 => (),
            _ => {
                result = 0;
                left_break_idx = left_idx;
                break;
            }
        }
        match direction {
            Dir::Inc if left > right => {
                result = 0;
                left_break_idx = left_idx;
                break;
            }
            Dir::Dec if left < right => {
                result = 0;
                left_break_idx = left_idx;
                break;
            }
            _ => (),
        }
    }
    if result == 0 && retry {
        assert!(
            left_break_idx != line.len() - 1,
            "{line:?}, {left_break_idx}",
        );

        // retry with left dropped
        let retry_line = [
            line[0..left_break_idx].to_vec(),
            line[left_break_idx + 1..].to_vec(),
        ]
        .concat();
        if safe(&retry_line, false) == 1 {
            return 1;
        }

        // retry with right dropped
        if left_break_idx == line.len() - 2 {
            // drop last element in array
            safe(&line[0..left_break_idx + 1], false)
        } else {
            safe(
                &[
                    line[0..left_break_idx + 1].to_vec(),
                    line[left_break_idx + 2..].to_vec(),
                ]
                .concat(),
                false,
            )
        }
    } else {
        result
    }
}

fn maybe_safer(line: &[u32]) -> bool {
    let result = maybe_safe(line);
    let old = safe(line, true) != 0;
    assert_eq!(result, old, "{line:?}");
    result
}

fn maybe_safe(line: &[u32]) -> bool {
    if is_safe(line) {
        return true;
    }

    for idx in 0..line.len() {
        let retry_line = match idx {
            0 => line[1..].to_vec(),
            idx if idx == line.len() - 1 => line[0..idx].to_vec(),
            idx => [line[0..idx].to_vec(), line[idx + 1..].to_vec()].concat(),
        };
        if is_safe(&retry_line) {
            return true;
        }
    }
    false
}

fn is_safe(line: &[u32]) -> bool {
    assert!(line.len() >= 2);

    let direction = match (line[0], line[1]) {
        (left, right) if left < right => Dir::Inc,
        (left, right) if left > right => Dir::Dec,
        _ => return false,
    };
    for pair in line.windows(2) {
        let [left, right] = pair else {
            panic!("no pair found")
        };
        match left.abs_diff(*right) {
            1..=3 => (),
            _ => return false,
        }
        match direction {
            Dir::Inc if left > right => return false,
            Dir::Dec if left < right => return false,
            _ => (),
        }
    }
    true
}

fn main() {
    let input = read_input();
    println!("{}", puzzle_1(&input));
    println!("{}", puzzle_2(&input));
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
            vec![1, 5],
            vec![1, 4],
            vec![4, 1],
            vec![1, 2, 4, 7],     // increase by 1,2,3
            vec![1, 2, 4, 7, 11], // increase by 1,2,3,4: unsafe
        ];
        assert_eq!(crate::puzzle_1(&input), 5);
    }

    #[test]
    fn test_puzzle_2() {
        let input = vec![
            vec![7, 6, 4, 2, 1],     // safe
            vec![1, 2, 7, 8, 9],     // unsafe
            vec![9, 7, 6, 2, 1],     // unsafe
            vec![10, 13, 2, 14, 15], // inc, change direction, drop right
            vec![10, 3, 12, 14, 15], // inc, change direction, drop left
            vec![8, 6, 4, 4, 1],     // duplicate, drop left
            vec![1, 3, 6, 7, 9],     // safe
            vec![90, 92, 93, 91],    // inc, change direction, drop right
            vec![1, 5, 6],           // inc, step to large, drop left
            vec![1, 5, 2],           // inc, step to large, drop right
            vec![5, 1, 6],           // dec, step to large, drop right
            vec![5, 1, 2],           // dec, step to large, drop left
            vec![3, 3, 4],           // duplicate, drop left, start of line
            vec![3, 3, 3],           // unsafe
            vec![1, 2, 4, 7],        // increase by 1,2,3
            vec![92, 94, 97, 97],
            vec![12, 13, 10, 11, 11],
            vec![71, 69, 70, 71, 72, 75], // decr, then incr, drop first
        ];
        assert_eq!(crate::puzzle_2(&input), 14);
    }
}
