use std::{isize, usize};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Coord(pub isize, pub isize);

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Coord {
    pub fn new(row: isize, col: isize) -> Self {
        (row, col).into()
    }
    pub fn row(&self) -> usize {
        self.0 as _
    }

    pub fn irow(&self) -> isize {
        self.0
    }

    pub fn col(&self) -> usize {
        self.1 as _
    }

    pub fn icol(&self) -> isize {
        self.1
    }

    pub fn neighbors(&self) -> impl Iterator<Item = Self> {
        [
            (self.irow() - 1, self.icol()).into(),
            (self.irow() + 1, self.icol()).into(),
            (self.irow(), self.icol() - 1).into(),
            (self.irow(), self.icol() + 1).into(),
        ]
        .into_iter()
    }

    pub fn manhattan(&self, other: &Self) -> usize {
        ((self.irow() - other.irow()).abs() + (self.icol() - other.icol()).abs()) as usize
    }
}

impl From<(usize, usize)> for Coord {
    fn from(value: (usize, usize)) -> Self {
        Self(value.0 as _, value.1 as _)
    }
}

impl From<(isize, isize)> for Coord {
    fn from(value: (isize, isize)) -> Self {
        Self(value.0 as _, value.1 as _)
    }
}

impl From<(i32, i32)> for Coord {
    fn from(value: (i32, i32)) -> Self {
        Self(value.0 as _, value.1 as _)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    pub(crate) fn turn_left(self) -> Self {
        match self {
            Dir::N => Self::W,
            Dir::S => Self::E,
            Dir::E => Self::N,
            Dir::W => Self::S,
        }
    }

    pub(crate) fn turn_right(self) -> Self {
        match self {
            Dir::N => Self::E,
            Dir::S => Self::W,
            Dir::E => Self::S,
            Dir::W => Self::N,
        }
    }
}

#[derive(Clone, Copy, Hash)]
pub struct Pos {
    pub dir: Dir,
    pub c: Coord,
}

impl Pos {
    pub fn move_forward(&self) -> Self {
        let dp = match self.dir {
            Dir::N => (-1, 0),
            Dir::S => (1, 0),
            Dir::E => (0, 1),
            Dir::W => (0, -1),
        };

        let c = Coord(self.c.0 + dp.0, self.c.1 + dp.1);

        Self { dir: self.dir, c }
    }

    pub fn row(&self) -> usize {
        self.c.row()
    }


    pub fn col(&self) -> usize {
        self.c.col()
    }
}
