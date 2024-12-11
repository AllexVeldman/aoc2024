use std::collections::HashMap;

trait Split {
    fn split(&self) -> Option<(u64, u64)>;
}

impl Split for u64 {
    /// Split the number in half
    /// so 1234 becomes (12, 34)
    /// returns None when the number does not have an even amount of digits
    fn split(&self) -> Option<(u64, u64)> {
        let len = self.ilog10() + 1;
        if len % 2 != 0 {
            return None;
        }
        let exp = 10_u64.pow(len / 2);
        Some((self / exp, self - self / exp * exp))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Stone(u64);

impl Stone {
    fn split(&self) -> Vec<Stone> {
        match &self.0 {
            number if *number == 0 => {
                vec![Stone(1)]
            }
            number => match number.split() {
                Some((left, right)) => vec![Stone(left), Stone(right)],
                None => vec![Stone(self.0 * 2024)],
            },
        }
    }
}

// apply blinks splits, giving back the amount the accumulator should increase
fn split(
    acc: usize,
    stone: Stone,
    blinks: usize,
    cache: &mut HashMap<(usize, Stone), usize>,
) -> usize {
    if let Some(value) = cache.get(&(blinks, stone.clone())) {
        return *value;
    }
    let mut internal_acc = acc;
    let stones = stone.split();
    if stones.len() == 2 {
        // split resulted in an extra stone
        internal_acc += 1;
    }
    if blinks == 1 {
        // we're at the end of the recursion, return
        cache.insert((blinks, stone), internal_acc - acc);
        return internal_acc - acc;
    }
    for stone in stones {
        // apply another split, acc already contains previous splits
        internal_acc += split(internal_acc, stone, blinks - 1, cache)
    }
    cache.insert((blinks, stone), internal_acc - acc);
    internal_acc - acc
}

impl From<&str> for Stone {
    fn from(value: &str) -> Self {
        Stone(value.parse::<u64>().unwrap())
    }
}

fn rumble(input: &str) -> Vec<Stone> {
    input.trim().split(' ').map(|f| f.into()).collect()
}

fn puzzle_1(input: &[Stone], blinks: usize) -> usize {
    let input = input.to_owned();
    let mut acc = 0;
    let mut cache = HashMap::new();
    for stone in input {
        acc += 1;
        acc += split(acc, stone, blinks, &mut cache);
    }
    acc
}

fn main() {
    let input = rumble("3028 78 973951 5146801 5 0 23533 857");
    println!("{}", puzzle_1(&input, 75));
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(&rumble("12"), 1), 2);
        let input = rumble("125 17");
        assert_eq!(puzzle_1(&input, 6), 22);
        assert_eq!(puzzle_1(&input, 25), 55312);
    }

    #[test]
    fn test_log_len() {
        assert_eq!(1234_u64.split(), Some((12, 34)));
        assert_eq!(123456_u64.split(), Some((123, 456)));
    }
}
