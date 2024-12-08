use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

#[derive(Debug, PartialEq, Eq)]
struct Antenna {
    freq: char,
    pos: Pos,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pos(isize, isize);

impl From<(usize, usize)> for Pos {
    fn from((row, col): (usize, usize)) -> Self {
        Self(row as isize, col as isize)
    }
}

impl Add for &Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for &Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Antenna {
    fn new(freq: char, row: usize, col: usize) -> Self {
        Antenna {
            freq,
            pos: Pos(row as isize, col as isize),
        }
    }

    /// Return the locations of the antinodes
    /// returns None if the other antenna is a different frequency
    fn anti_nodes(&self, other: &Antenna) -> Option<(Pos, Pos)> {
        if self.freq != other.freq || self == other {
            return None;
        }
        let diff = &self.pos - &other.pos;
        let node_1 = &self.pos + &diff;
        let node_2 = &other.pos - &diff;
        Some((node_1, node_2))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Antennas {
    antennas: Vec<Antenna>,
    max_row: usize,
    max_col: usize,
}

impl Antennas {
    fn new(input: &str) -> Self {
        let mut antennas = Vec::new();
        let mut max_row = 0;
        let mut max_col = 0;
        for (row, line) in input.trim().lines().enumerate() {
            max_row = row;
            for (col, ch) in line.chars().enumerate() {
                max_col = col;
                match ch {
                    '.' => {}
                    freq => antennas.push(Antenna::new(freq, row, col)),
                };
            }
        }
        Antennas {
            antennas,
            max_row,
            max_col,
        }
    }
    fn valid(&self, position: &Pos) -> bool {
        position.0 <= self.max_row as isize
            && position.1 <= self.max_col as isize
            && position.0 >= 0
            && position.1 >= 0
    }
    fn anti_nodes(&self, lhs: &Antenna, rhs: &Antenna) -> Option<HashSet<Pos>> {
        if lhs.freq != rhs.freq || lhs == rhs {
            return None;
        }
        let mut anti_nodes = HashSet::from([lhs.pos.clone(), rhs.pos.clone()]);
        let diff = &lhs.pos - &rhs.pos;

        // find the downstream nodes
        let mut node = &lhs.pos + &diff;
        loop {
            if !self.valid(&node) {
                break;
            }
            anti_nodes.insert(node.clone());
            node = &node + &diff;
        }

        // find the downstream nodes
        let mut node = &rhs.pos - &diff;
        loop {
            if !self.valid(&node) {
                break;
            }
            anti_nodes.insert(node.clone());
            node = &node - &diff;
        }

        Some(anti_nodes)
    }

    fn print(&self, anti_nodes: &HashSet<Pos>) {
        for row_idx in 0..=self.max_row {
            let mut row = vec!['.'; self.max_row + 1];
            for ant in self.antennas.iter() {
                // antennas are always inside the map
                if ant.pos.0 as usize == row_idx {
                    row[ant.pos.1 as usize] = ant.freq;
                }
            }

            // Ensure the node is inside the map
            for node in anti_nodes.iter().filter(|f| self.valid(f)) {
                if node.0 as usize == row_idx {
                    row[node.1 as usize] = '#';
                }
            }

            println!("{}", row.iter().collect::<String>());
        }
    }
}

fn puzzle_1(antennas: &Antennas) -> usize {
    let mut anti_nodes: HashSet<Pos> = HashSet::new();

    for ant in antennas.antennas.iter() {
        for other_ant in antennas.antennas.iter() {
            match ant.anti_nodes(other_ant) {
                None => {}
                Some((pos_1, pos_2)) => {
                    if antennas.valid(&pos_1) {
                        anti_nodes.insert(pos_1);
                    }
                    if antennas.valid(&pos_2) {
                        anti_nodes.insert(pos_2);
                    }
                }
            }
        }
    }
    // for node in anti_nodes.iter() {
    //     println!("{node:?}");
    // }

    anti_nodes.len()
}

fn puzzle_2(antennas: &Antennas) -> usize {
    let mut anti_nodes: HashSet<Pos> = HashSet::new();

    for ant in antennas.antennas.iter() {
        for other_ant in antennas.antennas.iter() {
            match antennas.anti_nodes(ant, other_ant) {
                None => {}
                Some(nodes) => {
                    anti_nodes.extend(nodes);
                }
            }
        }
    }
    // antennas.print(&anti_nodes);
    // for node in anti_nodes.iter() {
    //     println!("{node:?}");
    // }

    anti_nodes.len()
}

fn main() {
    let input = r#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
        "#;
    let input = std::fs::read_to_string("input.txt").unwrap();

    let antennas = Antennas::new(&input);
    println!("{}", puzzle_1(&antennas));
    println!("{}", puzzle_2(&antennas));
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_input() {
        let input = r#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
        "#;

        let antennas = Antennas::new(input);
        assert_eq!(
            antennas.antennas,
            vec![
                Antenna::new('0', 1, 8),
                Antenna::new('0', 2, 5),
                Antenna::new('0', 3, 7),
                Antenna::new('0', 4, 4),
                Antenna::new('A', 5, 6),
                Antenna::new('A', 8, 8),
                Antenna::new('A', 9, 9),
            ]
        );

        assert_eq!(puzzle_1(&antennas), 14);
        assert_eq!(puzzle_2(&antennas), 34);
    }

    #[test]
    fn test_anti_nodes() {
        // #.......
        // ........
        // ..0.....
        // ........
        // ....0...
        // ........
        // ......#.
        // ........
        let result = Antenna::new('0', 4, 4)
            .anti_nodes(&Antenna::new('0', 2, 2))
            .unwrap();
        assert_eq!(result, (Pos(6, 6), Pos(0, 0)));

        let result = Antenna::new('0', 2, 2)
            .anti_nodes(&Antenna::new('0', 4, 4))
            .unwrap();
        assert_eq!(result, (Pos(0, 0), Pos(6, 6)));

        let result = Antenna::new('0', 4, 4)
            .anti_nodes(&Antenna::new('0', 4, 3))
            .unwrap();
        assert_eq!(result, (Pos(4, 5), Pos(4, 2)));
    }

    #[test]
    fn test_pos_add() {
        let lhs = Pos(4, 4);
        let rhs = Pos(2, 2);

        assert_eq!(&lhs + &rhs, Pos(6, 6));
        assert_eq!(&lhs - &rhs, Pos(2, 2));
    }
}
