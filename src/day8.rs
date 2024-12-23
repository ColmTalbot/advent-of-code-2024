use std::collections::{HashMap, HashSet};
use num::complex::Complex;

fn read_data(kind: &str) -> Vec<Vec<char>> {
    std::fs::read_to_string(&format!("data/{kind}/day8.txt")).unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

pub fn part1(kind: &str) -> usize {
    let grid = read_data(kind);
    let mut antennae: HashMap<char, Vec<Complex<i32>>> = HashMap::new();

    // loop through the grid and keep track of all of the antennae (any non . character)
    // labelled by the character and a list of the positions they are found at
    for (xi, row) in grid.iter().enumerate() {
        for (yi, cell) in row.iter().enumerate() {
            if *cell != '.' {
                let position = Complex::new(xi as i32, yi as i32);
                let antenna = antennae.entry(*cell).or_insert(Vec::new());
                antenna.push(position);
            }
        }
    }

    let mut antinodes: HashSet<Complex<i32>> = HashSet::new();
    let xsize = grid.len() as i32;
    let ysize = grid[0].len() as i32;
    // for each class of antenna find all pairs and add negative of the vector connecting
    // them to each of the two antennae. add each point to the set of antinodes if it is
    // within the grid
    for antenna in antennae.values() {
        for i in 0..antenna.len() {
            for j in i+1..antenna.len() {
                let diff = antenna[j] - antenna[i];
                let mut position = antenna[j] + diff;
                if position.re >= 0 && position.im >= 0 && position.re < xsize && position.im < ysize {
                    antinodes.insert(position);
                }
                position = antenna[i] - diff;
                if position.re >= 0 && position.im >= 0 && position.re < xsize && position.im < ysize {
                    antinodes.insert(position);
                }
            }
        }
    }

    antinodes.len()
}

pub fn part2(kind: &str) -> usize {
    let grid = read_data(kind);
    let mut antennae: HashMap<char, Vec<Complex<i32>>> = HashMap::new();

    // loop through the grid and keep track of all of the antennae (any non . character)
    // labelled by the character and a list of the positions they are found at
    for (xi, row) in grid.iter().enumerate() {
        for (yi, cell) in row.iter().enumerate() {
            if *cell != '.' {
                let position = Complex::new(xi as i32, yi as i32);
                let antenna = antennae.entry(*cell).or_insert(Vec::new());
                antenna.push(position);
            }
        }
    }

    let mut antinodes: HashSet<Complex<i32>> = HashSet::new();
    let xsize = grid.len() as i32;
    let ysize = grid[0].len() as i32;
    // for each class of antenna find all pairs and add negative of the vector connecting
    // them to each of the two antennae while still in the grid. add each point to the
    // set of antinodes if it is within the grid
    for antenna in antennae.values() {
        for i in 0..antenna.len() {
            antinodes.insert(antenna[i]);
            for j in i+1..antenna.len() {
                let diff = antenna[j] - antenna[i];
                let mut position = antenna[j] + diff;
                while position.re >= 0 && position.im >= 0 && position.re < xsize && position.im < ysize {
                    antinodes.insert(position);
                    position += diff;
                }
                position = antenna[i] - diff;
                while position.re >= 0 && position.im >= 0 && position.re < xsize && position.im < ysize {
                    antinodes.insert(position);
                    position -= diff;
                }
            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("test"), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test"), 34);
    }
}