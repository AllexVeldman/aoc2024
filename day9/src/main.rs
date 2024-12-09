use std::env;

fn mangle(input: &str) -> Vec<u32> {
    input
        .trim()
        .chars()
        .map(|f| {
            f.to_digit(10)
                .unwrap_or_else(|| panic!("Not a base 10 digit: {}", f))
        })
        .collect()
}

/// "1234"
/// id: 0, 1 block
/// 2 empty block
/// id: 1, 3 blocks
/// 4 empty blocks
///
/// So:
/// - even indices are file blocks
/// - odd indices are empty blocks
///
///
fn fragment(input: &[u32]) -> Vec<usize> {
    let mut result = Vec::new();

    let mut left_file_id = 0;

    let mut right_file_id = (input.len() + input.len() % 2) / 2 - 1;
    let mut right_iter = input.iter().rev();
    if input.len() % 2 == 0 {
        // End of the input is an empty block, skip it
        right_iter.next();
    }
    // step over the empty file blocks
    let mut right_iter = right_iter.step_by(2);
    let mut right_file = right_iter.next().unwrap().to_owned();

    for (left_idx, block_size) in input.iter().enumerate() {
        match left_idx % 2 {
            // odd, empty block
            1 => {
                for _ in 0..*block_size {
                    result.push(right_file_id);
                    right_file -= 1;
                    if right_file == 0 {
                        // get the next file block length from the end of the input
                        right_file = right_iter.next().unwrap().to_owned();
                        right_file_id -= 1;
                        if right_file_id < left_file_id {
                            break;
                        }
                    }
                }
            }
            // even, file block
            0 => {
                let mut block_size = *block_size;
                if left_file_id == right_file_id {
                    // we hit the same file block as what is currently be unwinded
                    // from the right.
                    // right_file determines how many positions we have left
                    block_size = right_file;
                }
                for _ in 0..block_size {
                    result.push(left_file_id);
                }
                left_file_id += 1;
            }
            _ => panic!("Unexpected modulo"),
        }
        if right_file_id < left_file_id {
            break;
        }
    }

    result
}

fn checksum(input: &[usize]) -> usize {
    let mut acc = 0;
    for (idx, value) in input.iter().enumerate() {
        acc += idx * value;
    }
    acc
}

fn puzzle_1(input: &[u32]) -> usize {
    checksum(&fragment(input))
}

fn main() {
    // let input = mangle(&env::args().nth(1).unwrap());
    let input = mangle(&std::fs::read_to_string("input.txt").unwrap());
    println!("{}", puzzle_1(&input));
}

#[cfg(test)]
mod test {
    #[test]
    fn test_fragment() {
        let input = crate::mangle("2333133121414131402");
        let result = crate::fragment(&input);
        assert_eq!(
            result,
            vec![
                0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5, 5, 5, 5, 6, 6
            ]
        );
    }

    #[test]
    fn test_puzzle_1() {
        let input = crate::mangle("2333133121414131402");
        let result = crate::puzzle_1(&input);
        assert_eq!(result, 1928);
    }
}
