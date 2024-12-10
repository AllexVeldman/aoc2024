use std::collections::{HashMap, HashSet};

/// Location of a position on the map
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Loc {
    row: usize,
    col: usize,
}

/// Single position on the map
#[derive(Debug)]
struct Node {
    height: u8,
    neighbors: HashSet<Loc>,
}

impl Node {
    fn new(height: u8) -> Self {
        Node {
            height,
            neighbors: HashSet::new(),
        }
    }
}

/// Graph representation of the trails on the map
#[derive(Debug, Default)]
struct TrailMap {
    // map of each location and it's properties
    nodes: HashMap<Loc, Node>,
    // list of starting points
    trailheads: HashSet<Loc>,
}

impl TrailMap {
    fn new(input: &str) -> Self {
        let mut trail_map = TrailMap::default();
        for (row, line) in input.trim().lines().enumerate() {
            for (col, height) in line
                .chars()
                .map(|c| c.to_digit(10).expect("Not a valid int"))
                .enumerate()
            {
                let height = height as u8;
                let loc = Loc { row, col };
                let mut node = Node::new(height);

                // Add left neighbor, if it has an appropriate height
                if col != 0 {
                    let neighbor_loc = Loc { row, col: col - 1 };
                    trail_map.update_neighbors(&loc, &mut node, neighbor_loc);
                }
                // Add left neighbor, if it has an appropriate height
                if row != 0 {
                    let neighbor_loc = Loc { row: row - 1, col };
                    trail_map.update_neighbors(&loc, &mut node, neighbor_loc);
                }

                trail_map.add_node(loc, node);
            }
        }
        trail_map
    }

    fn update_neighbors(&mut self, loc: &Loc, node: &mut Node, neighbor: Loc) {
        let neighbor_node = self
            .nodes
            .get_mut(&neighbor)
            .expect("neighbor should already exist");
        if node.height.abs_diff(neighbor_node.height) == 1 {
            node.neighbors.insert(neighbor);
            neighbor_node.neighbors.insert(loc.clone());
        }
    }

    fn add_node(&mut self, loc: Loc, node: Node) {
        if node.height == 0 {
            self.trailheads.insert(loc.clone());
        }
        self.nodes.insert(loc, node);
    }

    /// Walk a trail
    /// returns true if the trail has an end at height 9
    /// returns false if the trail is not a trailhead or does not end in height 9
    fn walk_trail(&self, trailhead: &Loc) -> Option<(usize, HashSet<&Loc>)> {
        match self.nodes.get(trailhead) {
            Some(node) if node.height != 0 => None,
            None => None,
            Some(_) => self.walk(trailhead),
        }
    }

    fn walk(&self, start: &Loc) -> Option<(usize, HashSet<&Loc>)> {
        let current_node = match self.nodes.get(start) {
            None => return None,
            Some(node) => node,
        };
        // Keeps all locations with a height of 9 that is reachable from start
        let mut end_of_trails = HashSet::new();
        // counts the number of distincts routes for a trailhead
        let mut rating = 0;
        for neighbor_loc in current_node.neighbors.iter() {
            let neighbor_node = match self.nodes.get(neighbor_loc) {
                // unknown location
                None => return None,
                // height increases
                Some(neighbor) if neighbor.height > current_node.height => neighbor,
                // height decreases, check the next neighbor
                _ => continue,
            };
            if neighbor_node.height == 9 {
                end_of_trails.insert(neighbor_loc);
                rating += 1;
            } else if let Some((subrating, ends)) = self.walk(neighbor_loc) {
                end_of_trails.extend(ends);
                rating += subrating;
            }
        }

        if end_of_trails.is_empty() {
            return None;
        }
        Some((rating, end_of_trails))
    }

    fn score(&self, trailhead: &Loc) -> usize {
        match self.walk_trail(trailhead) {
            None => 0,
            Some((_, ends)) => ends.len(),
        }
    }
    fn rating(&self, trailhead: &Loc) -> usize {
        match self.walk_trail(trailhead) {
            None => 0,
            Some((rating, _)) => rating,
        }
    }
}

fn puzzle_1(map: &TrailMap) -> usize {
    map.trailheads
        .iter()
        .fold(0, |acc, trailhead| acc + map.score(trailhead))
}

fn puzzle_2(map: &TrailMap) -> usize {
    map.trailheads
        .iter()
        .fold(0, |acc, trailhead| acc + map.rating(trailhead))
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let map = TrailMap::new(&input);
    println!("{}", puzzle_1(&map));
    println!("{}", puzzle_2(&map));
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_trail_map_small() {
        let input = r#"
0123
1234
8765
9876
    "#;
        let map = TrailMap::new(input);
        assert_eq!(
            map.nodes.get(&Loc { row: 0, col: 0 }).unwrap().neighbors,
            HashSet::from([Loc { row: 0, col: 1 }, Loc { row: 1, col: 0 }])
        );
        assert_eq!(
            map.nodes.get(&Loc { row: 2, col: 2 }).unwrap().neighbors,
            HashSet::from([
                Loc { row: 2, col: 1 },
                Loc { row: 2, col: 3 },
                Loc { row: 3, col: 2 }
            ])
        );
        assert_eq!(map.trailheads, HashSet::from([Loc { row: 0, col: 0 }]));
        let trailhead = map.trailheads.iter().next().unwrap();
        assert_eq!(
            map.walk_trail(trailhead).unwrap().1,
            HashSet::from([&Loc { col: 0, row: 3 }])
        );
        assert_eq!(map.score(trailhead), 1);
    }

    #[test]
    fn test_trail_map() {
        let input = r#"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
    "#;
        let map = TrailMap::new(input);

        assert_eq!(map.trailheads.len(), 9);
        assert_eq!(map.score(&Loc { row: 0, col: 2 }), 5);

        assert_eq!(puzzle_1(&map), 36);
        assert_eq!(puzzle_2(&map), 81);
    }
}
