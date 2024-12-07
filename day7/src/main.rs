use core::panic;
use std::ops::{Add, AddAssign};

#[derive(Clone, Debug, PartialEq)]
enum Operator {
    Add,
    Mul,
    Concat,
}

impl From<usize> for Operator {
    fn from(value: usize) -> Self {
        match value {
            0 => Operator::Add,
            1 => Operator::Mul,
            _ => panic!("Unsupported operator"),
        }
    }
}

impl Add<usize> for &Operator {
    type Output = (bool, Operator);

    fn add(self, rhs: usize) -> Self::Output {
        if rhs != 1 {
            todo!("We'll only increment by 1")
        }
        match self {
            Operator::Add => (false, Operator::Mul),
            Operator::Mul => (false, Operator::Concat),
            Operator::Concat => (true, Operator::Add),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Operation(Vec<Operator>);

impl AddAssign<usize> for Operation {
    fn add_assign(&mut self, rhs: usize) {
        if rhs != 1 {
            todo!("We'll only increment by 1")
        }

        for idx in 0..self.0.len() {
            let (overflow, opp) = &self.0[idx] + 1;
            self.0[idx] = opp;
            if !overflow {
                break;
            }
        }
    }
}

impl Operation {
    fn max(&self) -> bool {
        self.0 == vec![Operator::Concat; self.0.len()]
    }
}

struct Equation {
    result: usize,
    parts: Vec<usize>,
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        match value.splitn(2, ':').collect::<Vec<_>>()[..] {
            [expected_result, rest] => {
                let parts = rest.trim().split(' ').map(|f| f.parse().unwrap()).collect();
                Equation {
                    result: expected_result.parse().unwrap(),
                    parts,
                }
            }
            _ => panic!("multiple ':' in line"),
        }
    }
}

impl Equation {
    fn valid(&self) -> bool {
        let mut operations = Operation(vec![Operator::Add; self.parts.len() - 1]);

        loop {
            if self.test_opps(&operations.0) {
                return true;
            };
            if operations.max() {
                break;
            }
            operations += 1;
        }

        false
    }

    fn test_opps(&self, operations: &[Operator]) -> bool {
        println!("{:?} - {operations:?}", self.parts);
        let mut lhs = self.parts[0];
        for (idx, rhs) in self.parts[1..].iter().enumerate() {
            lhs = match operations[idx] {
                Operator::Add => lhs + rhs,
                Operator::Mul => lhs * rhs,
                Operator::Concat => format!("{lhs}{rhs}").parse().unwrap(),
            };
        }
        lhs == self.result
    }
}

fn mangle(data: &str) -> Vec<Equation> {
    let mut result = Vec::new();
    for line in data.lines() {
        if line.trim().is_empty() {
            continue;
        }
        result.push(line.into());
    }
    result
}

fn puzzle_1(eqs: &[Equation]) -> usize {
    eqs.iter()
        .filter(|f| f.valid())
        .fold(0, |acc, e| acc + e.result)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let data = mangle(&input);
    println!("{}", puzzle_1(&data));
}

#[cfg(test)]
mod test {
    use crate::puzzle_1;
    use crate::Operation;
    use crate::Operator::Add;
    use crate::Operator::Mul;

    #[test]
    fn test_eval() {
        let input = r#"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
        "#;

        let data = crate::mangle(input);
        let x = data.iter().map(|f| f.valid()).collect::<Vec<bool>>();
        assert_eq!(
            x,
            vec![true, true, false, false, false, false, false, false, true]
        );

        assert_eq!(puzzle_1(&data), 3749);
    }

    #[test]
    fn test_operation() {
        let mut opp = Operation(vec![Add; 4]);
        opp += 1;
        assert_eq!(opp, Operation(vec![Mul, Add, Add, Add]));
        opp += 1;
        assert_eq!(opp, Operation(vec![Add, Mul, Add, Add]));
        opp += 1;
        assert_eq!(opp, Operation(vec![Mul, Mul, Add, Add]));
        opp += 1;
        assert_eq!(opp, Operation(vec![Add, Add, Mul, Add]));
        opp += 1;
        assert_eq!(opp, Operation(vec![Mul, Add, Mul, Add]));

        let mut opp = Operation(vec![Mul; 4]);
        opp += 1;
        assert_eq!(opp, Operation(vec![Add, Add, Add, Add]));
    }
}
