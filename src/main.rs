#[macro_use]
extern crate lazy_static;
extern crate itertools;
extern crate regex;
extern crate chrono;

pub type Error = Box<std::error::Error>;

mod util;
pub use util::*;

mod constants;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    run_all();
}

pub fn run_all() {
    let runners: Vec<(usize, usize)> = vec![
        (1, 1),
        (1, 2),
        (2, 1),
        (2, 2),
        (3, 1),
        (3, 2),
        (4, 1),
        (4, 2),
        (5, 1),
        (5, 2),
    ];

    for (day, part) in runners.iter() {
        run(*day, *part);
    }
}

pub fn run(day: usize, part: usize) {
    let out: String = match (day, part) {
        (1, 1) => day01::Part1::solve(&constants::day_1_input()).to_string(),
        (1, 2) => day01::Part2::solve(&constants::day_1_input()).to_string(),
        (2, 1) => day02::Part1::solve(constants::day_2_input()).to_string(),
        (2, 2) => day02::Part2::solve(constants::day_2_input()).to_string(),
        (3, 1) => day03::Part1::solve(constants::day_3_input()).unwrap().to_string(),
        (3, 2) => day03::Part2::solve(constants::day_3_input()).unwrap().to_string(),
        (4, 1) => day04::Part1::solve(constants::day_4_input()).unwrap().to_string(),
        (4, 2) => day04::Part2::solve(constants::day_4_input()).unwrap().to_string(),
        (5, 1) => day05::Part1::solve(constants::day_5_input()).to_string(),
        (5, 2) => day05::Part2::solve(constants::day_5_input()).to_string(),
        (_, _) => panic!(format!("Day: {}, Part {} UNIMPLEMENTED", day, part)),
    };

    println!("Day: {:02}, Part: {:02} => {}", day, part, out);
}