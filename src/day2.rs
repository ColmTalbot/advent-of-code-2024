fn read_data(fname: &str) -> Vec<Vec<i32>> {
    let mut output: Vec<Vec<i32>> = Vec::new();

    let file = std::fs::read_to_string(fname).unwrap();
    for line in file.lines() {
        output.push(line.split_whitespace().map(|val| {val.parse::<i32>().unwrap()}).collect());
    }
    output
}

pub fn part1(kind: &str) -> i32 {
    let data = read_data(&format!("data/{kind}/day2.txt"));

    let mut output = 0;

    data.iter().for_each(|line| {
        if line_increases(line, 0) { output += 1; }
    });

    return output
}

pub fn part2(kind: &str) -> i32 {
    let data = read_data(&format!("data/{kind}/day2.txt"));

    let mut output = 0;

    data.iter().for_each(|line| {
        if line_increases(line, 1) { output += 1; }
    });

    return output
}

fn line_increases(line: &Vec<i32>, allowed: i32) -> bool {
    if allowed < 0 { return false; }
    if line.len() < 2 { return true; }

    if line_increases(&line[1..line.len()].to_vec(), allowed - 1) {
        return true
    }

    let sign = (line[1] - line[0]).signum();
    let mut diff: i32;

    if sign == 0 {
        return line_increases(&line[1..line.len()].to_vec(), allowed - 1);
    }

    for idx in 1..line.len() {
        diff = line[idx] - line[idx - 1];
        if (diff.signum() != sign) || diff.abs() > 3 {
            let mut after: Vec<i32> = line[0..idx].to_vec();
            after.append(&mut line[idx + 1..line.len()].to_vec());
            let mut before: Vec<i32> = line[0..idx - 1].to_vec();
            before.append(&mut line[idx..line.len()].to_vec());
            let pass = line_increases(&after, allowed - 1)
                          || line_increases(&before, allowed - 1);
            return pass;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("test"), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test"), 4);
    }
}