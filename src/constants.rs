use std::str::FromStr;

pub fn day_1_input() -> Vec<i64> {
    include_str!("../resources/day01part01.txt")
        .lines()
        .map(|s| i64::from_str(&s).unwrap())
        .collect()
}

pub fn day_2_input() -> Vec<String> {
    include_str!("../resources/day02part01.txt")
        .lines()
        .map(|s| s.to_owned())
        .collect()
}

pub fn day_3_input() -> Vec<String> {
    include_str!("../resources/day03part01.txt")
        .lines()
        .map(|s| s.to_owned())
        .collect()
}

pub fn day_4_input() -> &'static str {
    include_str!("../resources/day04part01.txt")
}