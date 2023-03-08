
use thiserror::Error;
use integer_sqrt::IntegerSquareRoot;

pub struct HexGrid<T> {
    tiles: Vec<T>,
}

impl<T> HexGrid<T>
where
    T: Default,
{
    pub fn new(size: usize) -> Self {
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

#[cfg(test)]
impl<T> HexGrid<T> {
    pub fn len(&self) -> usize {
        self.tiles.len()
    }
}

impl<T> HexGrid<T> {
    pub fn get(&self, cord: Cordinate) -> Option<&T> {
        self.tiles.get(cord.as_index())
    }

    fn index_distance(i:usize) -> i32 {
        ((-1+4*i as i32).integer_sqrt() + 3) / 6
    }

    pub fn index_to_cord(i: usize) -> Cordinate {
        let dist: i32 = Self::index_distance(i);

        todo!()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cordinate {
    q: i32,
    r: i32,
    s: i32,
}

impl Cordinate {
    pub fn from_basis_vec(direction: HexDirection) -> Self {
        match direction {
            HexDirection::North => Self {q:0, r:-1,s:1},
            HexDirection::NorthEast => Self {q:1, r:-1,s:0},
            HexDirection::SouthEast => Self {q:1, r:0,s:-1},
            HexDirection::South => Self {q:0, r:1,s:-1},
            HexDirection::SouthWest => Self {q:-1, r:1,s:0},
            HexDirection::NorthWest => Self {q:-1, r:0,s:1},
        }
    }

    pub fn from_cube(q: i32, r: i32, s: i32) -> Result<Self, CordinateError> {
        if q + r + s != 0 {
            return Err(CordinateError::InvalidCubeCord(q, r, s));
        }
        Ok(Self { q, r, s })
    }

    /// panics if the resulting usize would be greater than
    /// `usize::MAX`
    pub fn as_index(&self) -> usize {
        //find the lagest num thats the distance from center
        let dist: i32 = *[self.q, self.r, self.s].map(|v| v.abs()).iter().max().unwrap();

        if dist == 0 {
            return 0;
        };

        const T: bool = true;

        let ring_offset = 1 + 3 * dist * (dist - 1);

        let offset_around_ring = match (
            self.q == dist,
            self.r == dist,
            self.s == dist,
            self.q == -dist,
            self.r == -dist,
            self.s == -dist,
        ) {
            (_, _, _, _, T, _) => self.q,
            (T, _, _, _, _, _) => dist + (-self.s),
            (_, _, _, _, _, T) => (dist * 2) + self.r,
            (_, T, _, _, _, _) => (dist * 3) + (-self.q),
            (_, _, _, T, _, _) => (dist * 4) + self.s,
            (_, _, T, _, _, _) => (dist * 5) + (-self.r),
            _ => unreachable!(),
        };

        (ring_offset + offset_around_ring).try_into().unwrap()
    }

}

impl std::ops::Add for Cordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            q:self.q + rhs.q,
            r:self.r + rhs.r,
            s:self.s + rhs.s
        }
    }
}

impl std::ops::Sub for Cordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            q:self.q - rhs.q,
            r:self.r - rhs.r,
            s:self.s - rhs.s
        }
    }
}

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
    #[error("the cordinate [{0},{1},{2}] does ")]
    InvalidCubeCord(i32, i32, i32),
}
