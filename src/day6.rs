use std::collections::HashSet;
use num::complex::Complex;

fn check_bounds(position: Complex<i32>, data: &Vec<Vec<char>>) -> bool {
    position.re >= 0 && position.im >= 0 && position.re < data.len() as i32 && position.im < data[0].len() as i32
}

fn read_data(kind: &str) -> Vec<Vec<char>> {
    std::fs::read_to_string(&format!("data/{kind}/day6.txt")).unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

fn find_start(data: &Vec<Vec<char>>) -> Complex<i32> {
    for xi in 0..data.len() {
        for yi in 0..data[xi].len() {
            if data[xi][yi] == '^' {
                return Complex::new(xi as i32, yi as i32);
            }
        }
    }
    Complex::new(0, 0)
}

// advance the current position and direction accounting for boundaries and obstacles
fn advance(position: Complex<i32>, direction: Complex<i32>, data: &Vec<Vec<char>>) -> (Complex<i32>, Complex<i32>) {
    let mut new_position = position + direction;
    let mut direction = direction;
    if check_bounds(new_position, data) {
        while data[new_position.re as usize][new_position.im as usize] == '#'
        {
            new_position -= direction;
            direction *= Complex::new(0, -1);
            new_position += direction;
        }
    }
    (new_position, direction)
}

// Find all the multiplications in the data and sum them
pub fn part1(kind: &str) -> i32 {
    let data = read_data(kind);
    
    let mut position = find_start(&data);
    let mut direction = Complex::new(-1, 0);
    let mut places: HashSet<Complex<i32>> = HashSet::new();

    while check_bounds(position, &data) {
        places.insert(position);
        (position, direction) = advance(position, direction, &data);
    }
    places.len() as i32
}

// find a loop in the chain using the tortoise and hare algorithm
fn escape(initial_position: Complex<i32>, direction: Complex<i32>, data: &Vec<Vec<char>>) -> bool {

    fn step(state: [Complex<i32>; 4], data: &Vec<Vec<char>>) -> [Complex<i32>; 4] {
        let [mut hare, mut tortoise, mut direction, mut slow_direction] = state;
        (hare, direction) = advance(hare, direction, data);
        (hare, direction) = advance(hare, direction, data);
        (hare, direction) = advance(hare, direction, data);
        (hare, direction) = advance(hare, direction, data);
        (hare, direction) = advance(hare, direction, data);
        (hare, direction) = advance(hare, direction, data);
        (hare, direction) = advance(hare, direction, data);
        (tortoise, slow_direction) = advance(tortoise, slow_direction, data);
        [hare, tortoise, direction, slow_direction]
    }

    let mut state = [initial_position, initial_position, direction, direction];
    state = step(state, data);
    while check_bounds(state[0], data) {
        if (state[0] == state[1]) && (state[2] == state[3]) {
            return true;
        }
        state = step(state, data);
    }
    false
}

pub fn part2(kind: &str, verbose: bool) -> i32 {
    let mut data = read_data(kind);
    
    let mut position = find_start(&data);
    let mut previous = position;
    let mut direction = Complex::new(-1, 0);
    let mut previous_direction = direction;
    let mut places: HashSet<Complex<i32>> = HashSet::new();
    (position, direction) = advance(position, direction, &data);

    while check_bounds(position, &data) {
        (position, direction) = match data[position.re as usize][position.im as usize] {
            '^' => {
                previous = position;
                previous_direction = direction;
                advance(position, direction, &data)
            },
            '.' | 'O' | '+' => {
                data[position.re as usize][position.im as usize] = '#';
                if escape(previous, previous_direction, &data) {
                    places.insert(position);
                    data[position.re as usize][position.im as usize] = 'O';
                } else {
                    data[position.re as usize][position.im as usize] = '+';
                }
                previous = position;
                previous_direction = direction;
                advance(position, direction, &data)
            },
            _ => panic!("Unexpected character"),
        };
    }

    if verbose {
        for line in &data {
            let temp: String = line.into_iter().collect();
            println!("{:?}", temp);
        }    
    }
    println!("{:?} {:?} {:?}", data.len(), data[0].len(), places.contains(&find_start(&data)));

    places.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("test"), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test", true), 6);
    }
}