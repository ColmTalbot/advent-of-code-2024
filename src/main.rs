mod day1;
mod day2;
mod day3;

fn main() {
    println!("Day 1: {}, {}", day1::part1("real"), day1::part2("real", false));
    println!("Day 2: {}, {}", day2::part1("real"), day2::part2("real"));
    println!("Day 3: {}, {}", day3::part1("real"), day3::part2("real"));
}
