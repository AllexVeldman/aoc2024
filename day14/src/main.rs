use core::panic;
use std::{collections::HashMap, fmt::Display, thread::sleep, time::Duration};

fn cls() {
    print!("\x1B[2J\x1B[1;1H");
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Loc {
    // column
    x: usize,
    // row
    y: usize,
}

impl From<&str> for Loc {
    fn from(value: &str) -> Self {
        match value.splitn(2, ',').collect::<Vec<_>>()[..] {
            [x, y] => Self {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            },
            _ => panic!("unvalid location"),
        }
    }
}

struct Velocity {
    // column
    x: isize,
    // row
    y: isize,
}

impl From<&str> for Velocity {
    fn from(value: &str) -> Self {
        match value.splitn(2, ',').collect::<Vec<_>>()[..] {
            [x, y] => Self {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            },
            _ => panic!("unvalid location"),
        }
    }
}

struct Guard {
    position: Loc,
    velocity: Velocity,
}

impl Guard {
    fn step(&mut self, x_limit: usize, y_limit: usize) {
        let new_x = match self.position.x as isize + self.velocity.x {
            new_x if new_x > x_limit as isize => new_x as usize - x_limit - 1,
            new_x if new_x < 0 => (x_limit as isize + new_x + 1) as usize,
            new_x => new_x as usize,
        };
        let new_y = match self.position.y as isize + self.velocity.y {
            new_y if new_y > y_limit as isize => new_y as usize - y_limit - 1,
            new_y if new_y < 0 => (y_limit as isize + new_y + 1) as usize,
            new_y => new_y as usize,
        };
        self.position.x = new_x;
        self.position.y = new_y;
    }
}

enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

struct Map {
    width: usize,
    height: usize,
    guards: Vec<Guard>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut guarded = HashMap::new();
        for guard in self.guards.iter() {
            *guarded.entry(&guard.position).or_insert(0) += 1;
        }

        for row in 0..self.height {
            for col in 0..self.width {
                match guarded.get(&Loc { x: col, y: row }) {
                    None => write!(f, ".")?,
                    Some(n) => write!(f, "{n}")?,
                };
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn simulate(&mut self, steps: usize) {
        for n in 0..steps {
            for guard in self.guards.iter_mut() {
                guard.step(self.width - 1, self.height - 1);
            }
            cls();
            println!("Step: {n}");
            println!("{self}");
            sleep(Duration::from_millis(200));
        }
    }

    fn quadrant(&self, guard: &Guard) -> Option<Quadrant> {
        if guard.position.x < self.width / 2 {
            if guard.position.y < self.height / 2 {
                return Some(Quadrant::TopLeft);
            }
            if guard.position.y > self.height / 2 {
                return Some(Quadrant::BottomLeft);
            }
        }
        if guard.position.x > self.width / 2 {
            if guard.position.y < self.height / 2 {
                return Some(Quadrant::TopRight);
            }
            if guard.position.y > self.height / 2 {
                return Some(Quadrant::BottomRight);
            }
        }
        None
    }

    fn guards_per_quadrant(&self) -> [usize; 4] {
        let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
        for guard in self.guards.iter() {
            match self.quadrant(guard) {
                Some(Quadrant::TopLeft) => q1 += 1,
                Some(Quadrant::BottomLeft) => q2 += 1,
                Some(Quadrant::TopRight) => q3 += 1,
                Some(Quadrant::BottomRight) => q4 += 1,
                None => {}
            }
        }
        [q1, q2, q3, q4]
    }
}

fn mangle(input: &str, width: usize, height: usize) -> Map {
    let mut guards = Vec::new();
    for line in input.trim().lines() {
        match line
            .strip_prefix("p=")
            .unwrap()
            .splitn(2, " v=")
            .collect::<Vec<_>>()[..]
        {
            [position, velocity] => guards.push(Guard {
                position: position.into(),
                velocity: velocity.into(),
            }),
            _ => panic!("unknown line"),
        };
    }
    Map {
        width,
        height,
        guards,
    }
}

fn main() {
    let mut map = mangle(&std::fs::read_to_string("input.txt").unwrap(), 101, 103);

    map.simulate(100);

    let score = map
        .guards_per_quadrant()
        .into_iter()
        .reduce(|acc, f| acc * f)
        .unwrap();
    println!("{score}");
}
