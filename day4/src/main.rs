use std::{collections::HashSet, fs};

fn read_input() -> Vec<Vec<char>> {
    let data = fs::read_to_string("input.txt").unwrap();
    mangle(&data)
}

fn mangle(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|f| f.chars().collect())
        .collect()
}

fn puzzle_1(input: &[Vec<char>]) -> usize {
    let word = "XMAS".chars().collect::<Vec<char>>();
    let column_length = input.len() - word.len();
    let row_length = input[0].len() - word.len();

    let mut hits = HashSet::new();
    for row in 0..=column_length {
        for column in 0..=row_length {
            search_window(input, row, column, &word, &mut hits);
        }
    }
    hits.len()
}

fn puzzle_2(input: &[Vec<char>]) -> usize {
    let word = "MAS".chars().collect::<Vec<char>>();
    let mut word_reversed = word.clone();
    word_reversed.reverse();
    let column_length = input.len() - word.len();
    let row_length = input[0].len() - word.len();

    let mut hits = 0;
    for row in 0..=column_length {
        for column in 0..=row_length {
            let [left_2_right, right_2_left] = diagonals(input, word.len(), row, column);
            let l2r = (left_2_right == word) || (left_2_right == word_reversed);
            let r2l = (right_2_left == word) || (right_2_left == word_reversed);
            if l2r && r2l {
                hits += 1;
            }
        }
    }
    hits
}

/// Search the window for all possible positions of word
fn search_window(
    window: &[Vec<char>],
    height_offset: usize,
    width_offset: usize,
    word: &[char],
    hits: &mut HashSet<((usize, usize), (usize, usize))>,
) {
    // Check window height >= word length
    assert!(window.len() >= height_offset + word.len());

    // horizontal hits
    for (row_idx, line) in window[height_offset..].iter().enumerate().take(word.len()) {
        // Check window width >= word length
        assert!(
            line.len() >= width_offset + word.len(),
            "{line:?}, offset: {width_offset}, word: {word:?}"
        );
        let line_start = (height_offset + row_idx, width_offset);
        let line_end = (height_offset + row_idx, width_offset + word.len() - 1);
        let mut subline = line[width_offset..width_offset + word.len()].to_vec();
        if subline == *word {
            hits.insert((line_start, line_end));
        }
        subline.reverse();
        if subline == *word {
            hits.insert((line_start, line_end));
        }
    }

    // vertical hits
    for column in 0..word.len() {
        let line_start = (height_offset, width_offset + column);
        let line_end = (height_offset + word.len() - 1, width_offset + column);

        let mut subline = Vec::new();
        for line in window[height_offset..].iter().take(word.len()) {
            subline.push(line[width_offset + column]);
        }
        if subline == word {
            hits.insert((line_start, line_end));
        }
        subline.reverse();
        if subline == word {
            hits.insert((line_start, line_end));
        }
    }

    // diagonal hits
    let [mut left_2_right, mut right_2_left] =
        diagonals(window, word.len(), height_offset, width_offset);
    if left_2_right == word {
        hits.insert((
            (height_offset, width_offset),
            (
                height_offset + word.len() - 1,
                width_offset + word.len() - 1,
            ),
        ));
    }
    if right_2_left == word {
        hits.insert((
            (height_offset, width_offset + word.len() - 1),
            (height_offset + word.len() - 1, width_offset),
        ));
    }
    left_2_right.reverse();
    right_2_left.reverse();
    if left_2_right == word {
        hits.insert((
            (height_offset, width_offset),
            (
                height_offset + word.len() - 1,
                width_offset + word.len() - 1,
            ),
        ));
    }
    if right_2_left == word {
        hits.insert((
            (height_offset, width_offset + word.len() - 1),
            (height_offset + word.len() - 1, width_offset),
        ));
    }
}

/// Returns the diagonal chars in the window
fn diagonals(
    window: &[Vec<char>],
    size: usize,
    height_offset: usize,
    width_offset: usize,
) -> [Vec<char>; 2] {
    let mut left_2_right = Vec::new();
    let mut right_2_left = Vec::new();
    for (idx, line) in window[height_offset..].iter().enumerate().take(size) {
        left_2_right.push(line[width_offset..][idx]);
        right_2_left.push(line[width_offset..][size - 1 - idx]);
    }
    [left_2_right, right_2_left]
}

fn main() {
    let data = read_input();
    println!("{}", puzzle_1(&data));
    println!("{}", puzzle_2(&data));
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    #[test]
    fn test_puzzle_1() {
        let input = crate::mangle(
            r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#,
        );
        assert_eq!(crate::puzzle_1(&input), 18)
    }
    #[test]
    fn test_puzzle_2() {
        let input = crate::mangle(
            r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#,
        );
        assert_eq!(crate::puzzle_2(&input), 9)
    }

    #[test]
    fn test_diagonals() {
        let input = crate::mangle(
            r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#,
        );
        let result = crate::diagonals(&input, 4, 1, 0);
        assert_eq!(result, [vec!['M', 'S', 'X', 'M'], vec!['S', 'A', 'M', 'M']]);
        let result = crate::diagonals(&input, 4, 3, 0);
        assert_eq!(result, [vec!['A', 'S', 'A', 'M'], vec!['S', 'A', 'M', 'X']]);
        let result = crate::diagonals(&input, 4, 5, 2);
        assert_eq!(result, [vec!['A', 'M', 'S', 'A'], vec!['M', 'M', 'M', 'X']]);
    }

    #[test]
    fn test_search_window() {
        let input = crate::mangle(
            r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#,
        );
        let word: Vec<char> = "XMAS".to_string().chars().collect();
        let mut hits = HashSet::new();
        crate::search_window(&input, 1, 0, &word, &mut hits);
        assert_eq!(hits.len(), 0);
        let mut hits = HashSet::new();
        crate::search_window(&input, 1, 5, &word, &mut hits);
        assert_eq!(hits.len(), 1);
        let mut hits = HashSet::new();
        crate::search_window(&input, 7, 2, &word, &mut hits);
        assert_eq!(hits.len(), 1);
    }
}
