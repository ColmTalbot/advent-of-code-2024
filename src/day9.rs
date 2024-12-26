fn read_data(kind: &str) -> Vec<i32> {
    let raw_data = std::fs::read_to_string(&format!("data/{kind}/day9.txt")).unwrap();
    let split_data: Vec<&str> = raw_data.split("").collect();
    let data = split_data[1..split_data.len() - 1].iter().map(|val| val.parse::<i32>().unwrap()).collect();
    data
}

pub fn part1(kind: &str) -> i64 {
    let data = read_data(kind);
    let mut idx1: usize = 0;
    let mut idx2 = data.len() - 1;
    let mut idx: usize = 0;
    let mut value1 = data[idx1];
    let mut value2 = data[idx2];


    let mut total: i64 = 0;
    // two pointer search
    while idx1 < idx2 {
        while value1 == 0 {
            idx1 += 1;
            value1 = data[idx1];
        }
        while value2 == 0 {
            idx2 -= 2;
            value2 = data[idx2];
        }
        if idx1 % 2 == 0 {
            total += idx as i64 * idx1 as i64 / 2;
            value1 -= 1;
        } else {
            total += idx as i64 * idx2 as i64 / 2;
            value1 -= 1;
            value2 -= 1;
        }
        idx += 1;
    }
    total
}

pub fn part2(kind: &str) -> i64 {
    let mut data = read_data(kind);
    let mut values: Vec<i32> = vec![];
    let mut idx: usize = 1;

    let mut checksum = 0;
    let mut position = 0;
    for _ in 0..data[0] {
        values.push(0);
        position += 1;
    }
    while idx <= data.len() - 1 {
        let mut temp = data.len() - 1;
        let mut space = data[idx];
        while space > 0 {
            let mut accepted = false;
            while temp > idx {
                if data[temp] > 0 && data[temp] <= space {
                    let mut value = data[temp];
                    space -= value;
                    while value > 0 {
                        checksum += position as i64 * temp as i64 / 2;
                        value -= 1;
                        position += 1;
                        values.push(temp as i32 / 2);
                    }
                    data[temp] = -data[temp];
                    accepted = true;
                    break;
                }
                temp -= 2;
            }
            if !accepted {
                for _ in 0..space {
                    position += 1;
                    values.push(999);
                }
                break;
            }
        }
        idx += 1;
        let mut value = data[idx];
        if value < 0 {
            while value < 0 {
                values.push(999);
                position += 1;
                value += 1;
            }
        } else {
            while value > 0 {
                checksum += position as i64 * idx as i64 / 2;
                values.push(idx as i32 / 2);
                value -= 1;
                position += 1;
            }
        }
        idx += 1;
    }
    checksum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1("test"), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test"), 2858);
    }
}