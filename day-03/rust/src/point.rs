#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub y: usize,
    pub x: usize,
}

impl Point {
    pub fn neighbors<const N: usize>(self) -> impl Iterator<Item = Self> {
        [
            self.up_left::<N>(),
            self.up::<N>(),
            self.up_right::<N>(),
            self.left::<N>(),
            self.right::<N>(),
            self.down_left::<N>(),
            self.down::<N>(),
            self.down_right::<N>(),
        ]
        .into_iter()
        .flatten()
    }

    fn up_left<const N: usize>(self) -> Option<Self> {
        self.y
            .checked_sub(1)
            .zip(self.x.checked_sub(1))
            .map(|(y, x)| Self { y, x })
    }

    fn up<const N: usize>(self) -> Option<Self> {
        self.y.checked_sub(1).map(|y| Self { y, ..self })
    }

    fn up_right<const N: usize>(self) -> Option<Self> {
        self.y
            .checked_sub(1)
            .zip(self.x.bounded_add::<N>(1))
            .map(|(y, x)| Self { y, x })
    }

    pub fn left<const N: usize>(&self) -> Option<Self> {
        self.x.checked_sub(1).map(|x| Self { x, ..*self })
    }

    pub fn right<const N: usize>(&self) -> Option<Self> {
        self.x.bounded_add::<N>(1).map(|x| Self { x, ..*self })
    }

    fn down_left<const N: usize>(self) -> Option<Self> {
        self.y
            .bounded_add::<N>(1)
            .zip(self.x.checked_sub(1))
            .map(|(y, x)| Self { y, x })
    }

    fn down<const N: usize>(self) -> Option<Self> {
        self.y.bounded_add::<N>(1).map(|y| Self { y, ..self })
    }

    fn down_right<const N: usize>(self) -> Option<Self> {
        self.y
            .bounded_add::<N>(1)
            .zip(self.x.bounded_add::<N>(1))
            .map(|(y, x)| Self { y, x })
    }
}

trait BoundedAdd: Copy {
    fn bounded_add<const N: usize>(self, other: Self) -> Option<Self>;
}

impl BoundedAdd for usize {
    fn bounded_add<const N: usize>(self, other: Self) -> Option<Self> {
        let sum = self + other;
        match sum < N {
            true => Some(sum),
            false => None,
        }
    }
}
