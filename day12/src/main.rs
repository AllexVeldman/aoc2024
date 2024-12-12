use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    ops::{Add, Sub},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Loc {
    row: usize,
    col: usize,
}

impl Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.row, self.col)
    }
}

impl Loc {
    // Give the Loc to the left of this
    fn left(&self) -> Loc {
        Loc {
            row: self.row,
            col: self.col.saturating_sub(1),
        }
    }
    fn right(&self) -> Loc {
        Loc {
            row: self.row,
            col: self.col + 1,
        }
    }
    fn up(&self) -> Loc {
        Loc {
            row: self.row.saturating_sub(1),
            col: self.col,
        }
    }
    fn down(&self) -> Loc {
        Loc {
            row: self.row + 1,
            col: self.col,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
struct Plot {
    neighbors: HashSet<Loc>,
}

impl Display for Plot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "neighbors: ")?;
        for n in self.neighbors.iter() {
            write!(f, "{n}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Region {
    plots: HashMap<Loc, Plot>,
}

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (loc, plot) in self.plots.iter() {
            writeln!(f, "{loc}:")?;
            writeln!(f, "\t{plot}")?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Add<usize> for Dir {
    type Output = Dir;

    fn add(self, _rhs: usize) -> Self::Output {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}

impl Sub<usize> for Dir {
    type Output = Dir;

    fn sub(self, _rhs: usize) -> Self::Output {
        match self {
            Dir::Up => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Right => Dir::Up,
        }
    }
}

impl Region {
    /// Add a plot, assumes top-left to bottom-right iteration
    fn add_plot(&mut self, loc: Loc) {
        let mut new_plot = Plot::default();
        self.plots.entry(loc.left()).and_modify(|neighbor| {
            new_plot.neighbors.insert(loc.left());
            neighbor.neighbors.insert(loc);
        });
        self.plots.entry(loc.up()).and_modify(|neighbor| {
            new_plot.neighbors.insert(loc.up());
            neighbor.neighbors.insert(loc);
        });
        self.plots.insert(loc, new_plot);
    }

    /// Get all neighbors, including the starting loc
    fn neighbors<'a>(&'a self, loc: &'a Loc) -> HashSet<&'a Loc> {
        let mut neighbors = HashSet::new();
        let mut queue = VecDeque::from([loc]);

        loop {
            let loc = match queue.pop_front() {
                None => break,
                Some(loc) => loc,
            };
            if neighbors.contains(loc) {
                continue;
            }
            neighbors.insert(loc);
            queue.extend(self.plots.get(loc).unwrap().neighbors.iter());
        }

        neighbors
    }

    /// Get a list of distinct regions
    fn distinct(&self) -> Vec<Region> {
        let mut visited: HashSet<&Loc> = HashSet::new();
        let mut distinct_regions = Vec::new();

        for loc in self.plots.keys() {
            if visited.contains(loc) {
                continue;
            }
            let neighbors = self.neighbors(loc);
            let mut new_region_plots = HashMap::new();
            for neighbor in neighbors.iter() {
                new_region_plots.insert(**neighbor, self.plots.get(neighbor).unwrap().clone());
            }
            distinct_regions.push(Region {
                plots: new_region_plots,
            });

            visited.extend(neighbors.iter());
        }

        distinct_regions
    }

    fn area(&self) -> usize {
        self.plots.len()
    }

    fn perimeter(&self) -> usize {
        let mut perimeter = 0;
        for plot in self.plots.values() {
            perimeter += 4 - plot.neighbors.len();
        }
        perimeter
    }

    fn neighbor(&self, loc: &Loc, dir: &Dir) -> Option<Loc> {
        let plot = self.plots.get(loc).unwrap();
        match dir {
            Dir::Up if plot.neighbors.contains(&loc.up()) => Some(loc.up()),
            Dir::Down if plot.neighbors.contains(&loc.down()) => Some(loc.down()),
            Dir::Left if plot.neighbors.contains(&loc.left()) => Some(loc.left()),
            Dir::Right if plot.neighbors.contains(&loc.right()) => Some(loc.right()),
            _ => None,
        }
    }

    fn sides(&self) -> usize {
        let mut visited: HashSet<(Loc, Dir)> = HashSet::new();
        let mut sides = 0;
        // find a starting position
        let mut current_loc = None;
        let mut direction = Dir::Up;

        for loc in self.plots.keys() {
            if self.neighbor(loc, &Dir::Up).is_none() {
                current_loc = Some(*loc);
                break;
            }
        }
        let mut current_loc = current_loc.unwrap();

        loop {
            if visited.contains(&(current_loc, direction)) {
                println!("Looking for other sides");
                // We looped around, see if there are sides we haven't visited yet
                // this would indicate inner edges
                let mut were_done = true;
                for loc in self.plots.keys() {
                    for dir in [Dir::Up, Dir::Right, Dir::Down, Dir::Left] {
                        if self.neighbor(loc, &dir).is_none() && !visited.contains(&(*loc, dir)) {
                            direction = dir;
                            current_loc = *loc;
                            were_done = false;
                            break; // direction for loop
                        }
                    }
                    if !were_done {
                        break; // location for loop
                    }
                }
                if were_done {
                    println!("break: {current_loc}: {direction:?}");
                    break; // outer loop
                }
            }
            println!("{current_loc}: {direction:?}: {sides}");
            current_loc = match self.neighbor(&current_loc, &(direction + 1)) {
                None => {
                    // edge does not continue in the same direction, turn right
                    println!("Turn right");
                    visited.insert((current_loc, direction));
                    sides += 1;
                    direction = direction + 1;
                    current_loc
                }
                Some(loc) => match self.neighbor(&loc, &direction) {
                    None => {
                        // the edge continues
                        visited.insert((current_loc, direction));
                        loc
                    }
                    Some(loc) => {
                        // edge does not continue in the same direction, turn left
                        println!("Turn left");
                        visited.insert((current_loc, direction));
                        sides += 1;
                        direction = direction - 1;
                        loc
                    }
                },
            };
        }

        sides
    }
}

#[derive(Default, Debug)]
struct Garden {
    regions: HashMap<char, Region>,
}

impl Display for Garden {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (label, region) in self.regions.iter() {
            writeln!(f, "{label}")?;
            writeln!(f, "{region}")?;
        }
        Ok(())
    }
}

impl Garden {
    fn add_plot(&mut self, plot: char, loc: Loc) {
        // No existing, connecting, region for this plot exists. create a new one
        self.regions.entry(plot).or_default().add_plot(loc);
    }

    /// Walk the garden, plotting its regions
    /// walks in a radial pattern so we can catch regions that loop back
    fn walk(garden: &[Vec<char>]) -> Self {
        let mut garden_obj = Garden::default();
        for (row, line) in garden.iter().enumerate() {
            for (col, plot) in line.iter().enumerate() {
                garden_obj.add_plot(*plot, Loc { row, col });
            }
        }
        garden_obj
    }

    fn fence_price(&self) -> usize {
        let mut fence_price = 0;
        for region in self.regions.values() {
            for distinct_region in region.distinct() {
                fence_price += distinct_region.area() * distinct_region.perimeter();
            }
        }
        fence_price
    }

    fn bulk_discount_price(&self) -> usize {
        let mut fence_price = 0;
        for region in self.regions.values() {
            for distinct_region in region.distinct() {
                fence_price += distinct_region.area() * distinct_region.sides();
            }
        }
        fence_price
    }
}

fn mangle(input: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for line in input.trim().lines() {
        result.push(line.chars().collect());
    }
    result
}

fn puzzle_1(input: &[Vec<char>]) -> usize {
    let garden = Garden::walk(input);
    garden.fence_price()
}
fn puzzle_2(input: &[Vec<char>]) -> usize {
    let garden = Garden::walk(input);
    garden.bulk_discount_price()
}

fn main() {
    let input = mangle(&std::fs::read_to_string("input.txt").unwrap());
    println!("{}", puzzle_1(&input));
    println!("{}", puzzle_2(&input));
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_garden() {
        let input = r#"
AAAA
BBCD
BBCC
EBEC
        "#;
        let input = mangle(input);
        let garden = Garden::walk(&input);
        assert_eq!(garden.regions.len(), 5);
        assert_eq!(garden.regions.get(&'A').unwrap().plots.len(), 4);

        // Order comes from HashSet, which is unordered :)
        // assert_eq!(
        //     garden.regions.get(&'E').unwrap().distinct(),
        //     vec![
        //         Region {
        //             plots: HashMap::from([(Loc { row: 3, col: 2 }, Plot::default())])
        //         },
        //         Region {
        //             plots: HashMap::from([(Loc { row: 3, col: 0 }, Plot::default())])
        //         },
        //     ]
        // );
    }

    #[test]
    fn test_puzzle_1_small() {
        let input = r#"
AAAA
BBCD
BBCC
EEEC
        "#;
        let input = mangle(input);
        assert_eq!(puzzle_1(&input), 140);
    }
    #[test]
    fn test_puzzle_1_u_shape() {
        let input = r#"
ABBB
ABAB
AAAB
BBBB
        "#;
        let input = mangle(input);
        assert_eq!(puzzle_1(&input), 6 * 14 + 10 * 22);
    }
    #[test]
    fn test_puzzle_1() {
        let input = r#"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
        "#;
        let input = mangle(input);
        assert_eq!(puzzle_1(&input), 1930);
        assert_eq!(puzzle_2(&input), 1206);
    }

    #[test]
    fn test_region_sides() {
        let input = r#"
AAAA
ABBA
AAAA
        "#;
        let input = mangle(input);
        let garden = Garden::walk(&input);
        let region = garden.regions.get(&'A').unwrap();
        assert_eq!(region.sides(), 8);
    }
}
