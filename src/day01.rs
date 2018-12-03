use super::*;

use std::iter;
use std::collections::HashSet;

pub struct Part1<T>(::std::marker::PhantomData<T>);

impl<T> Solve<T> for Part1<T>
    where T: AsRef<[i64]> {
    type Output = i64;

    fn solve(input: T) -> <Self as Solve<T>>::Output {
        input.as_ref()
            .iter()
            .sum()
    }
}

pub struct Part2<T>(::std::marker::PhantomData<T>);

impl<T> Solve<T> for Part2<T>
    where T: AsRef<[i64]> {
    type Output = i64;

    fn solve(input: T) -> <Self as Solve<T>>::Output {
        let mut set: HashSet<i64> = HashSet::new();

        for i in iter::once(0).chain(iter::repeat_with(|| input.as_ref())
            .flat_map(|s| s.iter())
            .scan(0, |state, item| {
                *state += item;
                Some(*state)
            })) {

            if set.contains(&i) {
                return i;
            }

            set.insert(i);
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(3, Part1::solve(vec![1, 1, 1]));
        assert_eq!(0, Part1::solve(vec![1, 1, -2]));
        assert_eq!(-6, Part1::solve(vec![-1, -2, -3]));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, Part2::solve(vec![1, -1]));
        assert_eq!(10, Part2::solve(vec![3, 3, 4, -2, -4]));
        assert_eq!(5, Part2::solve(vec![-6, 3, 8, 5, -6]));
        assert_eq!(14, Part2::solve(vec![7, 7, -2, -7, -4]));
    }
}