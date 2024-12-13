use std::{
    collections::{BinaryHeap, HashMap},
    ops::Add,
};

/// A position on the playing field
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

type Step = Position;

struct Button {
    step: Step,
    cost: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    cost: usize,
    position: Position,
}

impl Ord for State {
    /// BinaryHeap is a max-heap so will pop the greatest.
    /// Flip ordering so self > other if self.cost < other.cost
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

// Mandatory impl for Ord
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(puzzle: &Puzzle) -> Option<usize> {
    let target = puzzle.target;
    let a_button = &puzzle.a;
    let b_button = &puzzle.b;

    // Map of position to the minimal cost to that position from the start node
    let mut dist: HashMap<Position, usize> = HashMap::new();

    let mut prio_queue = BinaryHeap::new();
    prio_queue.push(State {
        cost: 0,
        position: Position { x: 0, y: 0 },
    });

    let mut final_cost = None;
    while let Some(State { cost, position }) = prio_queue.pop() {
        // println!("{position:?}: {cost}");
        // We hit the target
        if position == target {
            final_cost = Some(cost);
            break;
        }

        // Already found a better path to the current position
        if cost > *dist.get(&position).unwrap_or(&usize::MAX) {
            continue;
        }

        // overshot the target
        if position.x > target.x || position.y > target.y {
            continue;
        }

        for (neighbor, step_cost) in [
            (position + a_button.step, a_button.cost),
            (position + b_button.step, b_button.cost),
        ] {
            let next = State {
                cost: cost + step_cost,
                position: neighbor,
            };
            if next.cost < *dist.get(&next.position).unwrap_or(&usize::MAX) {
                dist.insert(next.position, next.cost);
                prio_queue.push(next);
            }
        }
    }

    println!("Cost: {final_cost:?}");
    final_cost
}

struct Puzzle {
    a: Button,
    b: Button,
    target: Position,
}

fn mangle(input: &str) -> Vec<Puzzle> {
    let mut input_iter = input.trim().lines();
    let mut puzzles = Vec::new();
    loop {
        match input_iter.by_ref().take(3).collect::<Vec<_>>()[..] {
            [but_a, but_b, prize] => {
                let button_a = match but_a
                    .strip_prefix("Button A: X+")
                    .unwrap()
                    .splitn(2, ',')
                    .collect::<Vec<_>>()[..]
                {
                    [x, left] => {
                        let x: usize = x.parse().unwrap();
                        let y: usize = left.trim().strip_prefix("Y+").unwrap().parse().unwrap();
                        Button {
                            step: Step { x, y },
                            cost: 3,
                        }
                    }
                    _ => panic!("unexpected line"),
                };
                let button_b = match but_b
                    .strip_prefix("Button B: X+")
                    .unwrap()
                    .splitn(2, ',')
                    .collect::<Vec<_>>()[..]
                {
                    [x, left] => {
                        let x: usize = x.parse().unwrap();
                        let y: usize = left.trim().strip_prefix("Y+").unwrap().parse().unwrap();
                        Button {
                            step: Step { x, y },
                            cost: 1,
                        }
                    }
                    _ => panic!("unexpected line"),
                };
                let target = match prize
                    .strip_prefix("Prize: X=")
                    .unwrap()
                    .splitn(2, ',')
                    .collect::<Vec<_>>()[..]
                {
                    [x, left] => {
                        let x: usize = x.parse().unwrap();
                        let y: usize = left.trim().strip_prefix("Y=").unwrap().parse().unwrap();
                        Position { x, y }
                    }
                    _ => panic!("unexpected line"),
                };
                puzzles.push(Puzzle {
                    target,
                    a: button_a,
                    b: button_b,
                });
            }
            _ => panic!("Don't know what happend here.."),
        }
        if input_iter.next().is_none() {
            break;
        };
    }
    puzzles
}

fn puzzle_1(puzzles: &[Puzzle]) -> usize {
    puzzles
        .iter()
        .fold(0, |acc, puzzle| acc + shortest_path(puzzle).unwrap_or(0))
}

fn main() {
    let input = mangle(&std::fs::read_to_string("input.txt").unwrap());

    println!("{}", puzzle_1(&input)); // 31552
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_1() {
        let puzzle = Puzzle {
            a: Button {
                step: Step { x: 94, y: 34 },
                cost: 3,
            },
            b: Button {
                step: Step { x: 22, y: 67 },
                cost: 1,
            },
            target: Position { x: 8400, y: 5400 },
        };
        assert_eq!(shortest_path(&puzzle), Some(280));
    }

    #[test]
    fn test_2() {
        let puzzle = Puzzle {
            a: Button {
                step: Step { x: 26, y: 66 },
                cost: 3,
            },
            b: Button {
                step: Step { x: 67, y: 21 },
                cost: 1,
            },
            target: Position { x: 12748, y: 12176 },
        };
        assert_eq!(shortest_path(&puzzle), None);
    }

    #[test]
    fn test_3() {
        let puzzle = Puzzle {
            a: Button {
                step: Step { x: 17, y: 86 },
                cost: 3,
            },
            b: Button {
                step: Step { x: 84, y: 37 },
                cost: 1,
            },
            target: Position { x: 7870, y: 6450 },
        };
        assert_eq!(shortest_path(&puzzle), Some(200));
    }

    #[test]
    fn test_4() {
        let puzzle = Puzzle {
            a: Button {
                step: Step { x: 69, y: 23 },
                cost: 3,
            },
            b: Button {
                step: Step { x: 27, y: 71 },
                cost: 1,
            },
            target: Position { x: 18641, y: 10279 },
        };
        assert_eq!(shortest_path(&puzzle), None);
    }

    #[test]
    fn test_puzzle_1() {
        let input = mangle(
            r#"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
    "#,
        );

        assert_eq!(puzzle_1(&input), 480);
    }
}
