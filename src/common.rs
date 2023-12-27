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
pub struct Pos<A> {
    pub dir: Dir,
    pub c: (A, A),
}

impl<A: num::Signed + num::ToPrimitive + Copy> Pos<A> {
    pub fn move_forward(&self) -> Self {
        let dp = match self.dir {
            Dir::N => (-A::one(), A::zero()),
            Dir::S => (A::one(), A::zero()),
            Dir::E => (A::zero(), A::one()),
            Dir::W => (A::zero(), -A::one()),
        };

        let c = (self.c.0 + dp.0, self.c.1 + dp.1);

        Self { dir: self.dir, c }
    }

    pub fn manhattan(&self, other: &Self) -> usize {
        ((self.c.0 - other.c.0).abs() + (self.c.1 - other.c.1).abs())
            .to_usize()
            .unwrap()
    }

    pub fn neighbors<F>(&self, pred: F) -> Vec<(A, A)>
    where
        F: Fn(&(A, A)) -> bool,
    {
        [
            (self.c.0 - A::one(), self.c.1),
            (self.c.0 + A::one(), self.c.1),
            (self.c.0, self.c.1 - A::one()),
            (self.c.0, self.c.1 + A::one()),
        ]
        .into_iter()
        .filter(pred)
        .collect()
    }
}
