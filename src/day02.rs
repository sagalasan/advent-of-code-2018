use super::*;

use std::collections::HashMap;

pub struct Part1<T>(::std::marker::PhantomData<T>);

impl<T> Solve<T> for Part1<T>
    where T: AsRef<[String]> {
    type Output = u64;

    fn solve(input: T) -> <Self as Solve<T>>::Output {
        let (a, b): (u64, u64) = input.as_ref()
            .iter()
            .map(|s| duplicates(s))
            .map(|d| {
                let mut two: u64 = 0;
                let mut three: u64 = 0;

                for (_byte, count) in d.iter() {
                    if *count == 2 {
                        two += 1;
                    } else if *count == 3 {
                        three += 1;
                    }
                }

                let a = if two > 0 { 1 } else { 0 };
                let b = if three > 0 { 1 } else { 0 };

                (a, b)
            })
            .fold((0, 0), |(acc_a, acc_b), (a, b)| {
                (acc_a + a, acc_b + b)
            });

        a * b
    }
}

pub struct Part2<T>(::std::marker::PhantomData<T>);

impl<T> Solve<T> for Part2<T>
    where T: AsRef<[String]> {
    type Output = String;

    fn solve(input: T) -> <Self as Solve<T>>::Output {
        let strings = input.as_ref();

        for i in 0..(strings.len() - 1) {
            let string1 = &strings[i];
            for j in (i + 1)..strings.len() {
                let string2 = &strings[j];

                let common = common(&string1, &string2);

                if common.len() == string1.len() - 1 {
                    return common;
                }
            }
        }

        unreachable!()
    }
}

fn duplicates(string: &str) -> Vec<(u8, u64)> {
    let mut map: HashMap<u8, u64> = HashMap::new();

    for &byte in string.as_bytes() {
        *map.entry(byte)
            .or_insert(0) += 1;
    }

    map.iter()
        .filter(|(&_b, &c)| c == 2 || c == 3)
        .map(|(&b, &c)| (b, c))
        .collect()
}

fn common(string1: &str, string2: &str) -> String {
    string1.chars().zip(string2.chars())
        .filter(|(a, b)| a == b)
        .map(|(a, _b)| a)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![
            "abcdef".to_owned(),
            "bababc".to_owned(),
            "abbcde".to_owned(),
            "abcccd".to_owned(),
            "aabcdd".to_owned(),
            "abcdee".to_owned(),
            "ababab".to_owned(),
        ];

        assert_eq!(12, Part1::solve(&input));
    }

    #[test]
    fn test_part2() {
        let input = vec![
            "abcde".to_owned(),
            "fghij".to_owned(),
            "klmno".to_owned(),
            "pqrst".to_owned(),
            "fguij".to_owned(),
            "axcye".to_owned(),
            "wvxyz".to_owned(),
        ];

        assert_eq!("fgij", Part2::solve(&input));
    }
}