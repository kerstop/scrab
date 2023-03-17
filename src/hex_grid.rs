use std::fmt::Display;

use integer_sqrt::IntegerSquareRoot;
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct HexGrid<T> {
    tiles: Vec<T>,
}

impl<T> HexGrid<T>
where
    T: Default,
{
    pub fn new(size: i32) -> Self {
        // 1 7 19
        // 0 6 18
        // x +1
        let mut tiles: Vec<T> = Vec::new();

        if size == 0 {
            return Self { tiles };
        };

        tiles.push(Default::default());

        for i in 1..size {
            for _ in 0..(6 * i) {
                tiles.push(Default::default())
            }
        }

        HexGrid { tiles }
    }
}

impl<T> HexGrid<T> {
    pub fn get(&self, cord: &Cordinate) -> Option<&T> {
        self.tiles.get(Self::cordinate_to_usize(cord))
    }

    pub fn get_mut(&mut self, cord: &Cordinate) -> Option<&mut T> {
        self.tiles.get_mut(Self::cordinate_to_usize(cord))
    }

    fn cordinate_to_usize(cord: &Cordinate) -> usize {
        //find the lagest num thats the distance from center
        let dist: i32 = *[cord.q, cord.r, cord.s]
            .map(|v| v.abs())
            .iter()
            .max()
            .unwrap();

        if dist == 0 {
            return 0;
        };

        const T: bool = true;

        let ring_offset = 1 + 3 * dist * (dist - 1);

        let offset_around_ring = match (
            cord.q == dist,
            cord.r == dist,
            cord.s == dist,
            cord.q == -dist,
            cord.r == -dist,
            cord.s == -dist,
        ) {
            (_, _, _, _, T, _) => cord.q,
            (T, _, _, _, _, _) => dist + (-cord.s),
            (_, _, _, _, _, T) => (dist * 2) + cord.r,
            (_, T, _, _, _, _) => (dist * 3) + (-cord.q),
            (_, _, _, T, _, _) => (dist * 4) + cord.s,
            (_, _, T, _, _, _) => (dist * 5) + (-cord.r),
            _ => unreachable!(),
        };

        (ring_offset + offset_around_ring).try_into().unwrap()
    }

    fn usize_to_cordinate(offset: usize) -> Cordinate {
        if offset == 0 {
            return Cordinate::new(0, 0, 0).unwrap();
        }

        let dist: i32 = (((-1 + 4 * offset as i32) * 3).integer_sqrt() + 3) / 6;

        let offset_of_ring = 1 + 3 * dist * (dist - 1);

        let offset_around_ring = offset as i32 - offset_of_ring;

        let side = offset_around_ring / dist;

        let constant = dist;
        let increasing = offset_around_ring - side * dist;
        let decreasing = constant - increasing;

        match side {
            0 => Cordinate::new(increasing, -constant, decreasing).unwrap(),
            1 => Cordinate::new(constant, -decreasing, -increasing).unwrap(),
            2 => Cordinate::new(decreasing, increasing, -constant).unwrap(),
            3 => Cordinate::new(-increasing, constant, -decreasing).unwrap(),
            4 => Cordinate::new(-constant, decreasing, increasing).unwrap(),
            5 => Cordinate::new(-decreasing, -increasing, constant).unwrap(),
            _ => unreachable!(),
        }
    }

    pub fn cordinates(&self) -> Cordinates {
        Cordinates {
            indexes: 0..self.tiles.len(),
        }
    }
}

// An iterator over the cordinates in a `HexGrid`
pub struct Cordinates {
    indexes: std::ops::Range<usize>,
}

impl Iterator for Cordinates {
    type Item = Cordinate;

    fn next(&mut self) -> Option<Self::Item> {
        self.indexes.next().map(HexGrid::<()>::usize_to_cordinate)
    }
}

pub struct Iter<'a, T>
where
    T: 'a,
{
    iter: std::slice::Iter<'a, T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct IterMut<'a, T>
where
    T: 'a,
{
    iter: std::slice::IterMut<'a, T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cordinate {
    q: i32,
    r: i32,
    s: i32,
}

impl Cordinate {
    pub fn new(q: i32, r: i32, s: i32) -> Result<Self, CordinateError> {
        if q + r + s != 0 {
            return Err(CordinateError::InvalidCubeCord(q, r, s));
        }
        Ok(Self { q, r, s })
    }

    pub fn to_pixel(&self, scale: f64) -> (i32, i32) {
        let x: i32 = (scale * (1.5 * f64::from(self.q))) as i32;
        let y: i32 = (scale
            * (3.0_f64.sqrt() / 2.0 * f64::from(self.q) + 3.0_f64.sqrt() * f64::from(self.r)))
            as i32;
        (x, y)
    }

    pub fn magnitude(&self) -> i32 {
        *[self.q, self.r, self.s]
            .map(|v| v.abs())
            .iter()
            .max()
            .unwrap()
    }
}

impl Display for Cordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{},{},{}]", self.q, self.r, self.s))
    }
}

impl From<HexDirection> for Cordinate {
    fn from(value: HexDirection) -> Self {
        match value {
            HexDirection::North => Self { q: 0, r: -1, s: 1 },
            HexDirection::NorthEast => Self { q: 1, r: -1, s: 0 },
            HexDirection::SouthEast => Self { q: 1, r: 0, s: -1 },
            HexDirection::South => Self { q: 0, r: 1, s: -1 },
            HexDirection::SouthWest => Self { q: -1, r: 1, s: 0 },
            HexDirection::NorthWest => Self { q: -1, r: 0, s: 1 },
        }
    }
}

impl std::ops::Add for Cordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
            s: self.s + rhs.s,
        }
    }
}

impl std::ops::Sub for Cordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
            s: self.s - rhs.s,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HexDirection {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

#[derive(Error, Debug)]
pub enum CordinateError {
    #[error("the cordinate [{0},{1},{2}] is invalid ")]
    InvalidCubeCord(i32, i32, i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn place_and_read() {
        let mut grid: HexGrid<i32> = HexGrid::new(11);

        *grid.get_mut(&Cordinate::new(8, 2, -10).unwrap()).unwrap() = 1;
        *grid.get_mut(&Cordinate::new(-3, 7, -4).unwrap()).unwrap() = 2;
        *grid.get_mut(&Cordinate::new(7, -9, 2).unwrap()).unwrap() = 3;
        *grid.get_mut(&Cordinate::new(8, -8, 0).unwrap()).unwrap() = 4;
        *grid.get_mut(&Cordinate::new(4, 5, -9).unwrap()).unwrap() = 5;

        assert_eq!(grid.get(&Cordinate::new(8, 2, -10).unwrap()).unwrap(), &1);
        assert_eq!(grid.get(&Cordinate::new(-3, 7, -4).unwrap()).unwrap(), &2);
        assert_eq!(grid.get(&Cordinate::new(7, -9, 2).unwrap()).unwrap(), &3);
        assert_eq!(grid.get(&Cordinate::new(8, -8, 0).unwrap()).unwrap(), &4);
        assert_eq!(grid.get(&Cordinate::new(4, 5, -9).unwrap()).unwrap(), &5);
    }

    #[test]
    fn new_sizes() {
        let grid_zero = HexGrid::<()>::new(0);
        let grid_one = HexGrid::<()>::new(1);
        let grid_two = HexGrid::<()>::new(2);
        let grid_three = HexGrid::<()>::new(3);
        let grid_four = HexGrid::<()>::new(4);
        let grid_five = HexGrid::<()>::new(5);

        assert_eq!(grid_zero.tiles.len(), 0);
        assert_eq!(grid_one.tiles.len(), 1);
        assert_eq!(grid_two.tiles.len(), 7);
        assert_eq!(grid_three.tiles.len(), 19);
        assert_eq!(grid_four.tiles.len(), 37);
        assert_eq!(grid_five.tiles.len(), 61);
    }

    #[test]
    fn cordinate_to_index() {
        vec![
            (0, 0, 0),
            (0, -1, 1),
            (1, -1, 0),
            (1, 0, -1),
            (0, 1, -1),
            (-1, 1, 0),
            (-1, 0, 1),
            (0, -2, 2),
            (1, -2, 1),
            (2, -2, 0),
            (2, -1, -1),
            (2, 0, -2),
            (1, 1, -2),
            (0, 2, -2),
            (-1, 2, -1),
            (-2, 2, 0),
            (-2, 1, 1),
            (-2, 0, 2),
            (-1, -1, 2),
            (0, -3, 3),
            (1, -3, 2),
            (2, -3, 1),
            (3, -3, 0),
            (3, -2, -1),
            (3, -1, -2),
            (3, 0, -3),
            (2, 1, -3),
            (1, 2, -3),
            (0, 3, -3),
            (-1, 3, -2),
            (-2, 3, -1),
            (-3, 3, 0),
            (-3, 2, 1),
            (-3, 1, 2),
            (-3, 0, 3),
            (-2, -1, 3),
            (-1, -2, 3),
        ]
        .iter()
        .enumerate()
        .for_each(|(i, (q, r, s))| {
            let cord = Cordinate::new(*q, *r, *s).unwrap();
            assert_eq!(HexGrid::<()>::cordinate_to_usize(&cord), i);
        });
    }

    #[test]
    fn index_to_cordinate() {
        vec![
            (0, 0, 0),
            (0, -1, 1),
            (1, -1, 0),
            (1, 0, -1),
            (0, 1, -1),
            (-1, 1, 0),
            (-1, 0, 1),
            (0, -2, 2),
            (1, -2, 1),
            (2, -2, 0),
            (2, -1, -1),
            (2, 0, -2),
            (1, 1, -2),
            (0, 2, -2),
            (-1, 2, -1),
            (-2, 2, 0),
            (-2, 1, 1),
            (-2, 0, 2),
            (-1, -1, 2),
            (0, -3, 3),
            (1, -3, 2),
            (2, -3, 1),
            (3, -3, 0),
            (3, -2, -1),
            (3, -1, -2),
            (3, 0, -3),
            (2, 1, -3),
            (1, 2, -3),
            (0, 3, -3),
            (-1, 3, -2),
            (-2, 3, -1),
            (-3, 3, 0),
            (-3, 2, 1),
            (-3, 1, 2),
            (-3, 0, 3),
            (-2, -1, 3),
            (-1, -2, 3),
        ]
        .iter()
        .enumerate()
        .for_each(|(i, (q, r, s))| {
            assert_eq!(
                HexGrid::<()>::usize_to_cordinate(i),
                Cordinate::new(*q, *r, *s).unwrap()
            )
        });
    }

    #[test]
    fn cordinates_iterator() {
        let mut cords_fixed = vec![
            (0, 0, 0),
            (0, -1, 1),
            (1, -1, 0),
            (1, 0, -1),
            (0, 1, -1),
            (-1, 1, 0),
            (-1, 0, 1),
            (0, -2, 2),
            (1, -2, 1),
            (2, -2, 0),
            (2, -1, -1),
            (2, 0, -2),
            (1, 1, -2),
            (0, 2, -2),
            (-1, 2, -1),
            (-2, 2, 0),
            (-2, 1, 1),
            (-2, 0, 2),
            (-1, -1, 2),
            (0, -3, 3),
            (1, -3, 2),
            (2, -3, 1),
            (3, -3, 0),
            (3, -2, -1),
            (3, -1, -2),
            (3, 0, -3),
            (2, 1, -3),
            (1, 2, -3),
            (0, 3, -3),
            (-1, 3, -2),
            (-2, 3, -1),
            (-3, 3, 0),
            (-3, 2, 1),
            (-3, 1, 2),
            (-3, 0, 3),
            (-2, -1, 3),
            (-1, -2, 3),
        ]
        .into_iter()
        .map(|c| Cordinate::new(c.0, c.1, c.2).unwrap());
        let mut cords_calculated = HexGrid::<()>::new(4).cordinates();

        loop {
            let c1 = cords_fixed.next();
            let c2 = cords_calculated.next();
            assert_eq!(c1, c2);

            if c1.is_none() {
                break;
            }
        }
    }
}
