mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    println!("Day 1: {}, {}", day1::part1("real"), day1::part2("real", false));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?} seconds", elapsed);
    let now = Instant::now();
    println!("Day 2: {}, {}", day2::part1("real"), day2::part2("real"));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?} seconds", elapsed);
    let now = Instant::now();
    println!("Day 3: {}, {}", day3::part1("real"), day3::part2("real"));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?} seconds", elapsed);
    let now = Instant::now();
    println!("Day 4: {}, {}", day4::part1("real"), day4::part2("real"));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?} seconds", elapsed);
    let now = Instant::now();
    println!("Day 5: {}, {}", day5::part1("real"), day5::part2("real", false));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?} seconds", elapsed);
    let now = Instant::now();
    println!("Day 6: {}, {}", day6::part1("real"), day6::part2("real", false));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?} seconds", elapsed);
    let now = Instant::now();
    println!("Day 7: {}, {}", day7::part1("real"), day7::part2("real"));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?} seconds", elapsed);
    let now = Instant::now();
    println!("Day 8: {}, {}", day8::part1("real"), day8::part2("real"));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?} seconds", elapsed);
    let now = Instant::now();
    println!("Day 9: {}, {}", day9::part1("real"), day9::part2("real"));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?} seconds", elapsed);
    let now = Instant::now();
    println!("Day 10: {}, {}", day10::part1("real"), day10::part2("real"));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?} seconds", elapsed);
    let now = Instant::now();
    println!("Day 11: {}, {}", day11::part1("real"), day11::part2("real"));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?} seconds", elapsed);

}
