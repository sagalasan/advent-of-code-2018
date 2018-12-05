use super::*;

pub struct Part1<T>(::std::marker::PhantomData<T>);

impl<T> Solve<T> for Part1<T>
    where T: AsRef<str> {
    type Output = usize;

    fn solve(input: T) -> <Self as Solve<T>>::Output {
        react_polymer(input.as_ref().chars()).len()
    }
}

pub struct Part2<T>(::std::marker::PhantomData<T>);

impl<T> Solve<T> for Part2<T>
    where T: AsRef<str> {
    type Output = usize;

    fn solve(input: T) -> <Self as Solve<T>>::Output {
        let pairs: Vec<(char, char)> = "abcdefghijklmnopqrstuvwxyz".chars()
            .map(|c| (c, c.to_uppercase().nth(0).unwrap()))
            .collect();

        let chars: Vec<char> = input.as_ref().chars().collect();

        pairs.iter()
            .map(|(lower, upper)| {
                chars.iter()
                    .filter(|&c| c != lower)
                    .filter(|&c| c != upper)
                    .map(|c| *c)
                    .collect::<Vec<char>>()
            })
            .map(|chars| react_polymer(chars).len())
            .min()
            .unwrap_or(0)
    }
}

fn react_polymer<T>(input: T) -> Vec<char>
    where T: IntoIterator<Item = char> {

    let mut a: Vec<char> = input.into_iter().collect();
    let mut b: Vec<char> = Vec::new();

    loop {
        let mut destroyed: usize = 0;

        let mut i: usize = 0;
        while i < a.len() - 1 {
            if (a[i] as isize - a[i + 1] as isize).abs() == 32 {
                destroyed += 1;
                i += 1;
            } else {
                b.push(a[i]);
            }

            i += 1;
        }

        if i == a.len() - 1 {
            b.push(a[i]);
        }

        if destroyed == 0 {
            return b;
        }

        std::mem::swap(&mut a, &mut b);
        b.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(10, Part1::solve("dabAcCaCBAcCcaDA"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(4, Part2::solve("dabAcCaCBAcCcaDA"));
    }
}