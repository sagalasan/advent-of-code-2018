use super::*;

use std::fmt;
use std::str::FromStr;
use std::iter;

use regex::Regex;

pub struct Part1<T>(::std::marker::PhantomData<T>);

impl<T> Solve<T> for Part1<T>
    where T: AsRef<[String]> {

    type Output = Result<usize, Error>;

    fn solve(input: T) -> <Self as Solve<T>>::Output {
        let claims: Vec<Claim> = input.as_ref().iter()
            .map(|s| Claim::from_str(s))
            .collect::<Result<Vec<Claim>, _>>()?;

        let max_size = claims.iter()
            .map(|c| {
                (c.top + c.height).max(c.left + c.width)
            })
            .max().ok_or("No claims")? as usize;

        let mut grid = Grid::square(max_size);

        for claim in claims.iter() {
            grid.add_claim(claim)?;
        }

        Ok(grid.overlapping())
    }
}

pub struct Part2<T>(::std::marker::PhantomData<T>);

impl <T> Solve<T> for Part2<T>
    where T: AsRef<[String]> {

    type Output = Result<usize, Error>;

    fn solve(input: T) -> <Self as Solve<T>>::Output {
        let claims: Vec<Claim> = input.as_ref().iter()
            .map(|s| Claim::from_str(s))
            .collect::<Result<Vec<Claim>, _>>()?;

        let max_size = claims.iter()
            .map(|c| {
                (c.top + c.height).max(c.left + c.width)
            })
            .max().ok_or("No claims")? as usize;

        let mut grid = Grid::square(max_size);

        for claim in claims.iter() {
            grid.add_claim(claim)?;
        }

        for claim in claims.iter() {
            if !grid.is_overlapping(claim)? {
                return Ok(claim.id);
            }
        }

        unreachable!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl Claim {
    pub fn new(id: usize, left: usize, top: usize, width: usize, height: usize) -> Self {
        Self {
            id,
            left,
            top,
            width,
            height,
        }
    }
}

impl FromStr for Claim {
    type Err = Box<::std::error::Error>;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#(\d+?) @ (\d+?),(\d+?): (\d+?)x(\d+)").unwrap();
        }

        let caps = RE.captures(s).ok_or("Invalid regex")?;

        let id = caps.get(1).ok_or("Invalid capture")?.as_str().parse::<usize>()?;
        let left = caps.get(2).ok_or("Invalid capture")?.as_str().parse::<usize>()?;
        let top = caps.get(3).ok_or("Invalid capture")?.as_str().parse::<usize>()?;
        let width = caps.get(4).ok_or("Invalid capture")?.as_str().parse::<usize>()?;
        let height = caps.get(5).ok_or("Invalid capture")?.as_str().parse::<usize>()?;

        Ok(Self::new(id, left, top, width, height))
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum GridItem {
    Empty,
    Occupied(usize),
    Overlapping,
}

impl fmt::Display for GridItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            GridItem::Empty => write!(f, "."),
            GridItem::Occupied(id) => write!(f, "{}", id),
            GridItem::Overlapping => write!(f, "X"),
        };

        Ok(())
    }
}

pub struct Grid {
    inner: Vec<Vec<GridItem>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    pub fn square(size: usize) -> Self {
        Self::new(size, size)
    }

    pub fn new(rows: usize, cols: usize) -> Self {
        let inner: Vec<Vec<GridItem>> = iter::repeat_with(|| vec![GridItem::Empty; cols])
            .take(rows)
            .collect();

        Self {
            inner,
            rows,
            cols
        }
    }

    pub fn get(&mut self, row: usize, col: usize) -> Option<&mut GridItem> {
        let mut inner = match self.inner.get_mut(row) {
            Some(inner) => inner,
            None => return None,
        };

        inner.get_mut(col)
    }

    pub fn add_claim(&mut self, claim: &Claim) -> Result<(), Error> {
        for row in claim.top..(claim.top + claim.height) {
            for col in claim.left..(claim.left + claim.width) {

                let item = self.get(row, col).ok_or("Invalid row or col")?;

                *item = match item {
                    GridItem::Empty => GridItem::Occupied(claim.id),
                    GridItem::Occupied(_) | GridItem::Overlapping => GridItem::Overlapping,
                };
            }
        }

        Ok(())
    }

    pub fn overlapping(&self) -> usize {
        self.inner.iter()
            .flat_map(|inner| inner.iter())
            .filter(|&item| *item == GridItem::Overlapping)
            .count()
    }

    pub fn is_overlapping(&mut self, claim: &Claim) -> Result<bool, Error> {
        for row in claim.top..(claim.top + claim.height) {
            for col in claim.left..(claim.left + claim.width) {

                let item = self.get(row, col).ok_or("Invalid row or col")?;

                if *item == GridItem::Overlapping {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for inner in self.inner.iter() {
            for item in inner {
                write!(f, "{}", item);
            }

            write!(f, "\n");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_claim() {
        let raw_claims = vec![
            "#1 @ 1,3: 4x4".to_owned(),
            "#2 @ 3,1: 4x4".to_owned(),
            "#3 @ 5,5: 2x2".to_owned(),
        ];

        let claims = vec![
            Claim::new(1, 1, 3, 4, 4),
            Claim::new(2, 3, 1, 4, 4),
            Claim::new(3, 5, 5, 2, 2),
        ];

        assert_eq!(claims[0], Claim::from_str(&raw_claims[0]).unwrap());
        assert_eq!(claims[1], Claim::from_str(&raw_claims[1]).unwrap());
        assert_eq!(claims[2], Claim::from_str(&raw_claims[2]).unwrap());
    }

    #[test]
    fn test_part1() {
        let raw_claims = vec![
            "#1 @ 1,3: 4x4".to_owned(),
            "#2 @ 3,1: 4x4".to_owned(),
            "#3 @ 5,5: 2x2".to_owned(),
        ];

        assert_eq!(4, Part1::solve(&raw_claims).unwrap());
    }

    #[test]
    fn test_part2() {
        let raw_claims = vec![
            "#1 @ 1,3: 4x4".to_owned(),
            "#2 @ 3,1: 4x4".to_owned(),
            "#3 @ 5,5: 2x2".to_owned(),
        ];

        assert_eq!(3, Part2::solve(&raw_claims).unwrap())
    }
}