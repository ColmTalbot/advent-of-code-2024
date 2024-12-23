fn read_data(kind: &str) -> (Vec<i64>, Vec<Vec<i64>>) {
    let mut results: Vec<i64> = vec![];
    let mut values: Vec<Vec<i64>> = vec![];
    let raw_data = std::fs::read_to_string(&format!("data/{kind}/day7.txt")).unwrap();
    let lines: Vec<&str> = raw_data.lines().collect();
    for line in lines {
            let split_line: Vec<&str> = line.split(": ").collect();
            results.push(split_line[0].parse::<i64>().unwrap());
            values.push(split_line[1].split(" ").map(|val| val.parse::<i64>().unwrap()).collect());
    };
    (results, values)
}

fn test_values_1(result: i64, values: &[i64]) -> bool {
    if values.len() == 1 {
        return values[0] == result;
    }
    let first = (values[0] + values[1], values[0] * values[1]);
    if values.len() == 2 {
        return (first.0 == result) || (first.1 == result);
    } else if (result < first.0) && (result < first.1) {
        return false;
    }
    let idx = values.len() - 1;
    let last = values[idx];
    (
        (result % last == 0) &&
        test_values_1(result / last, &values[0..idx])
    ) ||
    test_values_1(result - last, &values[0..idx])
}

fn concat(num1: i64, num2: i64) -> i64 {
    let mut temp = num1;
    let mut num_digits = 0;
    while temp > 0 {
        temp /= 10;
        num_digits += 1;
    }
    num2 * 10_i64.pow(num_digits) + num1
}

fn deconcat(result: i64, num: i64) -> (bool, i64) {
    let mut temp = num;
    let mut num_digits = 0;
    while temp > 0 {
        temp /= 10;
        num_digits += 1;
    }
    let pow_ten: i64 = 10_i64.pow(num_digits);
    if result % pow_ten == num {
        return (true, result / pow_ten);
    } else {
        return (false, result);
    }
}

fn test_values_2(result: i64, values: &[i64]) -> bool {
    if values.len() == 1 {
        return values[0] == result;
    }
    let first = (values[0] + values[1], values[0] * values[1], concat(values[1], values[0]));
    if values.len() == 2 {
        return (first.0 == result) || (first.1 == result) || (first.2 == result);
    } else if result <= 0 {
        return false;
    }
    let idx = values.len() - 1;
    let last = values[idx];
    let (deconcats, reduced) = deconcat(result, last);
    (deconcats && test_values_2(reduced, &values[0..idx]))
    || ((result % last == 0) && test_values_2(result / last, &values[0..idx]))
    || test_values_2(result - last, &values[0..idx])
}

pub fn part1(kind: &str) -> i64 {
    let (results, values) = read_data(kind);

    let mut total: i64 = 0;
    for (&result, value) in results.iter().zip(values.iter()) {
        if test_values_1(result, value) {
            total += result;
        }
    }

    total
}

pub fn part2(kind: &str) -> i64 {
    let (results, values) = read_data(kind);

    let mut total: i64 = 0;
    for (&result, value) in results.iter().zip(values.iter()) {
        if test_values_2(result, value) {
            println!("{}", result);
            total += result;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("test"), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test"), 11387);
    }
}