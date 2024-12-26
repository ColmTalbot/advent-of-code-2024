use std::collections::HashSet;
use num::complex::Complex;

fn check_bounds(position: Complex<i32>, data: &Vec<Vec<usize>>) -> bool {
    position.re >= 0 && position.im >= 0 && position.re < data.len() as i32 && position.im < data[0].len() as i32
}

fn read_data(kind: &str) -> Vec<Vec<usize>> {
    std::fs::read_to_string(&format!("data/{kind}/day10.txt")).unwrap()
        .lines()
        .map(|l| l.chars()
            .map(|l| l.to_digit(10).unwrap() as usize).collect()
        ).collect()
}

// fn find_start(data: &Vec<Vec<usize>) -> Complex<i32> {
//     for xi in 0..data.len() {
//         for yi in 0..data[xi].len() {
//             if data[xi][yi] == '^' {
//                 return Complex::new(xi as i32, yi as i32);
//             }
//         }
//     }
//     Complex::new(0, 0)
// }

// advance the current position and direction accounting for boundaries and obstacles
fn advance(position: Complex<i32>, direction: Complex<i32>, data: &Vec<Vec<usize>>) -> (Complex<i32>, Complex<i32>) {
    let mut new_position = position + direction;
    let mut direction = direction;
    if check_bounds(new_position, data) {
        while data[new_position.re as usize][new_position.im as usize] == 0
        {
            new_position -= direction;
            direction *= Complex::new(0, -1);
            new_position += direction;
        }
    }
    (new_position, direction)
}

fn climb(position: Complex<i32>, data: &Vec<Vec<usize>>) -> HashSet<Complex<i32>> {
    let mut places: HashSet<Complex<i32>> = HashSet::new();
    let value = data[position.re as usize][position.im as usize];
    if value == 9 {
        places.insert(position);
        return places;
    }
    for direction in [Complex::new(1, 0), Complex::new(-1, 0), Complex::new(0, 1), Complex::new(0, -1)].iter() {
        let new_position = position + direction;
        if check_bounds(new_position, data) && data[new_position.re as usize][new_position.im as usize] == value + 1 {
            places.extend(climb(new_position, data));
        }
    }
    places
}

pub fn part1(kind: &str) -> i32 {
    let data = read_data(kind);

    let mut total = 0;
    for xi in 0..data.len() {
        for yi in 0..data[xi].len() {
            if data[xi][yi] != 0 {
                continue;
            }
            let position = Complex::new(xi as i32, yi as i32);
            let places = climb(position, &data);
            total += places.len() as i32;
        }
    }
    total
}

fn descend(position: Complex<i32>, data: &Vec<Vec<usize>>) -> Vec<Complex<i32>> {
    let mut places: Vec<Complex<i32>> = vec![];
    let value = data[position.re as usize][position.im as usize];
    if value == 9 {
        places.push(position);
        return places;
    }
    for direction in [Complex::new(1, 0), Complex::new(-1, 0), Complex::new(0, 1), Complex::new(0, -1)].iter() {
        let new_position = position + direction;
        if check_bounds(new_position, data) && data[new_position.re as usize][new_position.im as usize] == value + 1 {
            places.extend(descend(new_position, data));
        }
    }
    places
}

pub fn part2(kind: &str) -> i32 {
    let data = read_data(kind);

    let mut total = 0;
    for xi in 0..data.len() {
        for yi in 0..data[xi].len() {
            if data[xi][yi] != 0 {
                continue;
            }
            let position = Complex::new(xi as i32, yi as i32);
            let places = descend(position, &data);
            total += places.len() as i32;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("test"), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test"), 81);
    }
}