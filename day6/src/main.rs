use std::{thread::sleep, time::Duration};

fn cls() {
    print!("\x1B[2J\x1B[1;1H");
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            other => panic!("unknown char {}", other),
        }
    }
}

impl From<Direction> for char {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

#[derive(Clone)]
struct Guard {
    row: usize,
    col: usize,
    dir: Direction,
}

#[derive(Clone)]
struct Map {
    map: Vec<Vec<char>>,
    guard: Guard,
}

enum EndGame {
    GameOver,
    Loop,
}

impl Map {
    fn print(&self) {
        for line in self.map.iter() {
            println!("{}", line.iter().collect::<String>());
        }
    }

    fn distinct_positions(&self) -> usize {
        let mut counter = 0;
        for line in self.map.iter() {
            for char in line.iter() {
                if !['.', '#'].contains(char) {
                    counter += 1;
                }
            }
        }
        counter
    }

    /// Move one tile
    fn move_guard(&mut self) -> Option<EndGame> {
        match self.guard.dir {
            Direction::Up => {
                if self.guard.row == 0 {
                    return Some(EndGame::GameOver);
                }
                self.guard.row -= 1;
            }
            Direction::Down => {
                self.guard.row += 1;
                if self.guard.row == self.map.len() {
                    return Some(EndGame::GameOver);
                };
            }
            Direction::Left => {
                if self.guard.col == 0 {
                    return Some(EndGame::GameOver);
                }
                self.guard.col -= 1;
            }
            Direction::Right => {
                self.guard.col += 1;
                if self.guard.col == self.map[0].len() {
                    return Some(EndGame::GameOver);
                }
            }
        };
        if self.map[self.guard.row][self.guard.col] == '#' {
            // invalid move, undo change, turn right, and try again
            match self.guard.dir {
                Direction::Up => {
                    self.guard.row += 1;
                    self.guard.dir = Direction::Right
                }
                Direction::Down => {
                    self.guard.row -= 1;
                    self.guard.dir = Direction::Left
                }
                Direction::Left => {
                    self.guard.col += 1;
                    self.guard.dir = Direction::Up
                }
                Direction::Right => {
                    self.guard.col -= 1;
                    self.guard.dir = Direction::Down
                }
            };
            return self.move_guard();
        }

        let new_location = self.map[self.guard.row][self.guard.col];
        match self.guard.dir {
            Direction::Up if new_location == '^' => {
                return Some(EndGame::Loop);
            }
            Direction::Down if new_location == 'v' => {
                return Some(EndGame::Loop);
            }
            Direction::Left if new_location == '<' => {
                return Some(EndGame::Loop);
            }
            Direction::Right if new_location == '>' => {
                return Some(EndGame::Loop);
            }
            _ => {}
        }
        self.map[self.guard.row][self.guard.col] = self.guard.dir.into();
        None
    }
}

fn mangle(input: &str) -> Map {
    let input = input.trim();
    let mut map = Vec::new();
    let mut guard: Option<Guard> = None;
    for (row, line) in input.lines().enumerate() {
        // Add the line to the map
        map.push(line.chars().collect::<Vec<char>>());
        // see if the starting position is on this line
        for (col, char) in line.chars().enumerate() {
            if ['<', '^', '>', 'v'].contains(&char) {
                guard = Some(Guard {
                    row,
                    col,
                    dir: char.into(),
                });
                break;
            }
        }
    }
    let guard = guard.expect("Did not find the guard");
    Map { map, guard }
}

fn puzzle_1(map: &mut Map) -> usize {
    while map.move_guard().is_none() {}
    map.distinct_positions()
}

fn puzzle_2(original_map: &mut Map) -> usize {
    let mut counter = 0;
    for (row, line) in original_map.map.iter().enumerate() {
        for (col, position) in line.iter().enumerate() {
            if *position != '#' && (row, col) != (original_map.guard.row, original_map.guard.col) {
                let mut map = original_map.clone();
                map.map[row][col] = '#';
                loop {
                    match map.move_guard() {
                        None => {
                            // sleep(Duration::from_millis(1));
                            // cls();
                            // println!("Loops: {counter}");
                            // map.print();
                        }
                        Some(EndGame::GameOver) => break,
                        Some(EndGame::Loop) => {
                            counter += 1;
                            break;
                        }
                    }
                }
            }
        }
    }
    counter
}

fn main() {
    let mut map = mangle(&std::fs::read_to_string("input.txt").unwrap());
    // println!("{}", puzzle_1(&mut map));

    let result = puzzle_2(&mut map);
    println!("Loops: {result}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_map() {
        let input = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;
        let mut map = mangle(input);
        map.print();
        while map.move_guard().is_none() {}
        assert_eq!(map.distinct_positions(), 41);
    }
}
