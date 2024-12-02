/// Read the input file like
// 
// 3   4
// 4   3
// 2   5
// 1   3
// 3   9
// 3   3
//
// and return a tuple of two vectors
fn read_day1(fname: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    let file = std::fs::read_to_string(fname).unwrap();
    for line in file.lines() {
        let mut iter = line.split_whitespace();
        left.push(iter.next().unwrap().parse().unwrap());
        right.push(iter.next().unwrap().parse().unwrap());
    }

    (left, right)
}

pub fn day1_part1(kind: &str) -> i32 {
    let (mut left, mut right) = read_day1(&format!("data/{kind}/day1.txt"));

    left.sort();
    right.sort();

    let mut total_distance: i32 = 0;

    left.iter().zip(right.iter()).for_each(|(l, r)| {
        total_distance += (l - r).abs();
    });

    return total_distance;
}

pub fn day1_part2(kind: &str, verbose: bool) -> i32 {
    let (mut left, mut right) = read_day1(&format!("data/{kind}/day1.txt"));

    left.sort();
    right.sort();

    let mut similarity: i32 = 0;

    let mut left_idx = 0;
    let mut right_idx = 0;
    let mut left_counts = 0;
    let mut right_counts = 0;
    let mut active = false;

    while left_idx < left.len() && right_idx < right.len() {
        if verbose {
            println!("{} {} {} {} {} {} {}", left_idx, right_idx, left[left_idx], right[right_idx], similarity, left_counts, right_counts);
        }
        if left[left_idx] == right[right_idx] {
            right_idx += 1;
            left_counts = left[left_idx];
            if active {right_counts += 1;}
            else {right_counts = 1;}
            active = true;
        } else if left[left_idx] < right[right_idx] {
            if left[left_idx] == left_counts {
                similarity += left[left_idx] * right_counts;
            }
            left_idx += 1;
            active = false;
        } else {
            right_idx += 1;
            active = false;
        }
    }

    return similarity;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_part1() {
        assert_eq!(day1_part1("test"), 11);
    }

    #[test]
    fn test_day1_part2() {
        assert_eq!(day1_part2("test", false), 31);
    }
}