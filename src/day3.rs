use regex::Regex;

// Find the multiplications in the data of the form "mul(abc,xyz)"
fn find_multiplications(data: &str) -> Vec<(&str, i32, i32)> {
    let re = Regex::new(r"(mul)\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut results = vec![];
    for (_, [op, first, second]) in re.captures_iter(&data).map(|c| c.extract()) {
        results.push((op, first.parse::<i32>().unwrap(), second.parse::<i32>().unwrap()));
    }
    results
}

// Find and sum the multiplications in the data
fn evaluate_str(data: &str) -> i32 {
    let mut total = 0;
    for (_, first, second) in find_multiplications(data) {
        total += first * second;
    }
    total
}

// Find all the multiplications in the data and sum them
pub fn part1(kind: &str) -> i32 {
    let data = std::fs::read_to_string(&format!("data/{kind}/day3.txt")).unwrap();
    evaluate_str(&data)
}

// Now we need to accomodate on/off switches in the form of do() and don't()
// We can do this by splitting the string on "don't()"
// The first part of the split will be evaluated as normal as we assume the switch is on
// at the beginning. The rest of the splits can then be split again on "do()" and
// the second part of the split can be evaluated as normal.
pub fn part2(kind: &str) -> i32 {
    let data = std::fs::read_to_string(&format!("data/{kind}/day3.txt")).unwrap();

    let split_data: Vec<&str> = data.split("don't()").collect();

    let mut total = evaluate_str(&split_data[0]);

    for idx in 1..split_data.len() {
        match split_data[idx].split_once("do()").unwrap_or(("", "")) {
            ("", "") => continue,
            (_, good) => total += evaluate_str(good),
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("test"), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test"), 48);
    }
}