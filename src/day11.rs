use std::{collections::{HashMap, HashSet}, hash::Hash};

fn read_data(kind: &str) -> Vec<i64> {
    std::fs::read_to_string(&format!("data/{kind}/day11.txt")).unwrap()
        .split(' ')
        .map(|l| l.parse().unwrap())
        .collect()
}

fn blink_single(value: i64) -> Vec<i64> {
    match value {
        0 => vec![1],
        x if x.ilog10() % 2 == 1 => {
            let digits = 10_i64.pow(x.ilog10() / 2 + 1);
            vec![value / digits, value % digits as i64]
        },
        _ => vec![2024 * value],
    }
}

fn blink(values: Vec<i64>) -> Vec<i64> {
    values.iter().map(|&v| blink_single(v)).flatten().collect()
}

// Find all the multiplications in the data and sum them
pub fn part1(kind: &str) -> usize {
    let mut data = read_data(kind);
    for _ in 0..25 {
        data = blink(data);
    }
    data.len()
}

fn blink_hash(values: HashMap<i64, usize>) -> HashMap<i64, usize> {
    let mut new_values = HashMap::new();
    for (value, count) in values.iter() {
        for new_value in blink_single(*value) {
            if new_values.contains_key(&new_value) {
                new_values.insert(new_value, new_values.get(&new_value).unwrap() + count);
            } else {
                new_values.insert(new_value, *count);
            }
        }
    }
    new_values
    // values.iter().map(|&v| blink_single(v)).flatten().collect()
}

pub fn part2(kind: &str) -> usize {
    let data = read_data(kind);
    let mut seen = HashMap::new();
    for value in data.iter() {
        if seen.contains_key(value) {
            seen.insert(*value, seen.get(value).unwrap() + 1);
        } else {
            seen.insert(*value, 1);
        }
    }
    for _ in 0..75 {
        seen = blink_hash(seen);
    }
    seen.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("test"), 55312);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test"), 65601038650482);
    }
}