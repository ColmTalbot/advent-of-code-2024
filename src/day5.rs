use std::collections::{HashMap, HashSet};

fn read_data(kind: &str) -> (HashMap<usize, HashSet<usize>>, Vec<Vec<usize>>) {
    let raw_data = std::fs::read_to_string(&format!("data/{kind}/day5.txt")).unwrap();
    let split_data: Vec<&str> = raw_data.split("\n\n").collect();
    let rules: Vec<&str> = split_data[0].split("\n").collect();
    let messages: Vec<&str> = split_data[1].split("\n").collect();
    let mut rule_map: HashMap<usize, HashSet<usize>> = HashMap::new();
    for rule in rules {
        let split_rule: Vec<&str> = rule.split("|").collect();
        let rule_num = split_rule[0].parse::<usize>().unwrap();
        let rule_def = split_rule[1].parse::<usize>().unwrap();
        if !rule_map.contains_key(&rule_num) {
            rule_map.insert(rule_num, HashSet::new());
        }
        rule_map.get_mut(&rule_num).unwrap().insert(rule_def);
    };
    let messages: Vec<Vec<usize>> = messages.iter()
        .map(|message| message.split(",")
            .map(|m| m.parse::<usize>().unwrap()).collect())
        .collect();
    (rule_map, messages)
}

fn message_is_sorted(message: &Vec<usize>, rule_map: &HashMap<usize, HashSet<usize>>) -> bool {
    for idx in 1..message.len() {
        if 
            !rule_map.contains_key(&message[idx - 1])
            || !rule_map.get(&message[idx - 1]).unwrap().contains(&message[idx]) {
                return false;
            }
    }
    true
}

fn sort_message(message: &Vec<usize>, rule_map: &HashMap<usize, HashSet<usize>>) -> Option<Vec<usize>> {
    let mut sorted = message.clone();
    let len = message.len();
    while !message_is_sorted(&sorted, &rule_map) {
        for idx in 1..len {
            if !rule_map.contains_key(&sorted[idx - 1]) {
                sorted.swap(idx - 1, len - 1);
                if !rule_map.contains_key(&sorted[idx - 1]) {
                    return None;
                }
            }
            let mut tidx = idx;
            while tidx < len && !rule_map.get(&sorted[tidx - 1]).unwrap().contains(&sorted[tidx]) {
                sorted.swap(tidx - 1, tidx);
                tidx += 1;
            }
        }
    }
    Some(sorted)
}

pub fn part1(kind: &str) -> usize {
    let (rule_map, messages) = read_data(kind);

    let mut total = 0;
    for message in messages {
        if message_is_sorted(&message, &rule_map) {
            total += message[message.len() / 2];
        }
    }
    total
}

pub fn part2(kind: &str, verbose: bool) -> usize {
    let (rule_map, messages) = read_data(kind);

    let mut total = 0;
    for message in messages {
        if message_is_sorted(&message, &rule_map) {
            continue;
        }
        let sorted = match sort_message(&message, &rule_map) {
            Some(answer) => answer,
            None => panic!("Unable to sort {:?}", message)
        };
        if verbose {
            println!("{} {:?} {:?}", sorted[sorted.len() / 2], message, sorted);
        }
        total += sorted[sorted.len() / 2];
    }
    total
}


// pub fn part2(kind: &str) -> i32 {
//     let (rule_map, messages) = read_data(kind);
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("test"), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test", true), 123);
    }
}