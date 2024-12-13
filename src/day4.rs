fn read_data(kind: &str) -> Vec<Vec<char>> {
    std::fs::read_to_string(&format!("data/{kind}/day4.txt")).unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

fn word_search(word: &str, data: &Vec<Vec<char>>, position: (i32, i32), direction: &str) -> i32 {
    if word.len() == 0 {
        return 1;
    }
    if position.0 < 0 || position.1 < 0 {
        return 0;
    }
    let x: usize = position.0 as usize;
    let y: usize = position.1 as usize;
    if x >= data.len() || y >= data[0].len() {
        return 0;
    }
    if data[x][y] != word.chars().next().unwrap() {
        return 0;
    }

    match direction {
        "" =>  word_search(&word[1..word.len()], data, (position.0 + 1, position.1), "up")
             + word_search(&word[1..word.len()], data, (position.0, position.1 + 1), "down")
             + word_search(&word[1..word.len()], data, (position.0 - 1, position.1), "left")
             + word_search(&word[1..word.len()], data, (position.0, position.1 - 1), "right")
             + word_search(&word[1..word.len()], data, (position.0 + 1, position.1 + 1), "upright")
             + word_search(&word[1..word.len()], data, (position.0 + 1, position.1 - 1), "downright")
             + word_search(&word[1..word.len()], data, (position.0 - 1, position.1 + 1), "upleft")
             + word_search(&word[1..word.len()], data, (position.0 - 1, position.1 - 1), "downleft"),
        "up" => word_search(&word[1..word.len()], data, (position.0 + 1, position.1), "up"),
        "down" => word_search(&word[1..word.len()], data, (position.0, position.1 + 1), "down"),
        "left" => word_search(&word[1..word.len()], data, (position.0 - 1, position.1), "left"),
        "right" => word_search(&word[1..word.len()], data, (position.0, position.1 - 1), "right"),
        "upright" => word_search(&word[1..word.len()], data, (position.0 + 1, position.1 + 1), "upright"),
        "downright" => word_search(&word[1..word.len()], data, (position.0 + 1, position.1 - 1), "downright"),
        "upleft" => word_search(&word[1..word.len()], data, (position.0 - 1, position.1 + 1), "upleft"),
        "downleft" => word_search(&word[1..word.len()], data, (position.0 - 1, position.1 - 1), "downleft"),
        _ => 0,
    }
}

fn cross_search(word: &str, data: &Vec<Vec<char>>, position: (usize, usize)) -> bool {
    let word = word.chars().collect::<Vec<char>>();
    if data[position.0][position.1] == word[1] {
        return ((
            data[position.0 - 1][position.1 - 1] == word[0] &&
            data[position.0 + 1][position.1 + 1] == word[2] 
        ) || (
            data[position.0 - 1][position.1 - 1] == word[2] &&
            data[position.0 + 1][position.1 + 1] == word[0]
        )) && ((
            data[position.0 + 1][position.1 - 1] == word[0] &&
            data[position.0 - 1][position.1 + 1] == word[2] 
        ) || (
            data[position.0 + 1][position.1 - 1] == word[2] &&
            data[position.0 - 1][position.1 + 1] == word[0]
        ))
    }
    false
}

// // Find and sum the multiplications in the data
// fn evaluate_str(data: &str) -> i32 {
//     let mut total = 0;
//     for (_, first, second) in find_multiplications(data) {
//         total += first * second;
//     }
//     total
// }

// Find all the multiplications in the data and sum them
pub fn part1(kind: &str) -> i32 {
    let data = read_data(kind);
    
    let mut total = 0;

    for xi in 0..data.len() as i32 {
        for yi in 0..data[xi as usize].len() as i32 {
            total += word_search("XMAS", &data, (xi, yi), "");
        }
    }
    total
}

// Now we need to accomodate on/off switches in the form of do() and don't()
// We can do this by splitting the string on "don't()"
// The first part of the split will be evaluated as normal as we assume the switch is on
// at the beginning. The rest of the splits can then be split again on "do()" and
// the second part of the split can be evaluated as normal.
pub fn part2(kind: &str) -> i32 {
    let data = read_data(kind);
    
    let mut total = 0;

    for xi in 1..(data.len() - 1) {
        for yi in 1..(data[xi].len() - 1) {
            if cross_search("MAS", &data, (xi, yi)) {
                total += 1;
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("test"), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test"), 9);
    }
}