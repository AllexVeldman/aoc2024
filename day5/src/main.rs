use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq, Eq)]
struct Rule(u32, u32);

fn read_input() -> (Vec<Rule>, Vec<Vec<u32>>) {
    let data = fs::read_to_string("input.txt").unwrap();
    mangle(&data)
}

impl FromIterator<u32> for Rule {
    fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> Self {
        match iter.into_iter().collect::<Vec<u32>>()[..] {
            [left, right] => Self(left, right),
            _ => panic!("invalid rule"),
        }
    }
}

fn mangle(input: &str) -> (Vec<Rule>, Vec<Vec<u32>>) {
    let mut rules: Vec<Rule> = Vec::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();
    let mut reading_rules = true;

    for (idx, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            if idx != 0 {
                reading_rules = false;
            }
            continue;
        }
        if reading_rules {
            let rule = line
                .splitn(2, '|')
                .map(|f| f.parse::<u32>().unwrap())
                .collect();
            rules.push(rule);
        } else {
            let update = line.split(',').map(|f| f.parse::<u32>().unwrap()).collect();
            updates.push(update);
        }
    }
    (rules, updates)
}

fn update_valid(rules: &[Rule], update: &[u32]) -> bool {
    let mut to_test: HashMap<u32, usize> = HashMap::new();
    for (idx, page) in update.iter().enumerate() {
        to_test.insert(*page, idx);
    }
    for rule in rules {
        match (to_test.get(&rule.0), to_test.get(&rule.1)) {
            (Some(left), Some(right)) if left >= right => return false,
            (Some(left), Some(right)) if left == right => panic!("Same page twice in rule"),
            _ => {}
        }
    }

    true
}

fn correct_order(rules: &[Rule], update: &[u32]) -> Vec<u32> {
    let mut sorted = Vec::new();

    for page in update {
        // Loop through all the sorted values
        let mut insertion_idx = 0;
        let mut break_sort = false;
        for (idx, sorted_value) in sorted.iter().enumerate() {
            for rule in rules {
                match rule {
                    // page should be inserted before current location
                    Rule(left, right) if left == page && right == sorted_value => {
                        // update the insertion index
                        insertion_idx = idx;
                        // break the sorted loop
                        break_sort = true;
                        // break the rule loop
                        break;
                    }
                    // page should be inserted after current location
                    Rule(left, right) if right == page && left == sorted_value => {
                        // set index at least after this point, once we hit a "before" rule or the
                        // end of the sorted list we know the insertion index
                        insertion_idx = idx + 1;
                        // break the rule loop
                        break;
                    }
                    // rule does not apply
                    _ => {}
                }
            }
            if break_sort {
                break;
            }
        }
        sorted.insert(insertion_idx, *page);
    }

    sorted
}

fn puzzle_1(rules: &[Rule], updates: &[Vec<u32>]) -> u32 {
    let mut acc = 0;
    for update in updates {
        if update_valid(rules, update) {
            assert!(
                update.len() % 2 != 0,
                "update has even number of pages, no middle page"
            );
            // integer division rounds to 0
            acc += update[update.len() / 2];
        }
    }
    acc
}

fn puzzle_2(rules: &[Rule], updates: &[Vec<u32>]) -> u32 {
    let mut acc = 0;
    for update in updates {
        if !update_valid(rules, update) {
            acc += correct_order(rules, update)[update.len() / 2]
        }
    }
    acc
}
fn main() {
    let (rules, updates) = read_input();
    println!("{}", puzzle_1(&rules, &updates));
    println!("{}", puzzle_2(&rules, &updates));
}

#[cfg(test)]
mod test {
    use crate::{puzzle_1, Rule};

    #[test]
    fn test_mangle() {
        let input = r#"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
        "#;
        let (rules, updates) = crate::mangle(input);
        assert_eq!(rules[0], crate::Rule(47, 53));
        assert_eq!(rules[20], crate::Rule(53, 13));

        assert_eq!(updates[0], vec![75, 47, 61, 53, 29]);
        assert_eq!(updates[5], vec![97, 13, 75, 29, 47]);

        assert!(crate::update_valid(&rules, &updates[0]));
        assert!(crate::update_valid(&rules, &updates[1]));
        assert!(crate::update_valid(&rules, &updates[2]));
        assert!(!crate::update_valid(&rules, &updates[3]));
        assert!(!crate::update_valid(&rules, &updates[4]));
        assert!(!crate::update_valid(&rules, &updates[5]));

        assert_eq!(crate::puzzle_1(&rules, &updates), 143);

        let mut ordered = Vec::new();
        for update in updates.iter() {
            if crate::update_valid(&rules, update) {
                ordered.push(update.clone());
            } else {
                ordered.push(crate::correct_order(&rules, update));
            }
        }

        assert_eq!(
            ordered,
            vec![
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13],
                vec![97, 75, 47, 61, 53],
                vec![61, 29, 13],
                vec![97, 75, 47, 29, 13],
            ]
        );

        assert_eq!(crate::puzzle_2(&rules, &updates), 123);
    }

    #[test]
    fn test_correct_order() {
        let result = crate::correct_order(
            &[Rule(61, 47), Rule(29, 75), Rule(61, 75), Rule(47, 53)],
            &[75, 47, 61, 53, 29],
        );
        assert_eq!(result, vec![61, 47, 53, 29, 75])
    }
}
