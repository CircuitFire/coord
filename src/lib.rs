use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Coord <T> {
    pub x: T,
    pub y: T,
}

impl<T: ops::Add<Output = T>> ops::Add for Coord<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: ops::AddAssign> ops::AddAssign for Coord<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: ops::Sub<Output = T>> ops::Sub for Coord<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: ops::SubAssign> ops::SubAssign for Coord<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T: ops::Rem<Output = T>> ops::Rem for Coord<T> {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Self {
            x: self.x % other.x,
            y: self.y % other.y,
        }
    }
}

impl<T: ops::RemAssign> ops::RemAssign for Coord<T> {
    fn rem_assign(&mut self, other: Self) {
        self.x %= other.x;
        self.y %= other.y;
    }
}

impl<T: ops::Mul<Output = T>> ops::Mul for Coord<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T: ops::Div<Output = T>> ops::Div for Coord<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl<T: PartialEq> PartialEq for Coord<T> {
    fn eq(&self, other: &Self) -> bool {
        if(self.x == other.x) && (self.y == other.x) { true }
        else { false }
    }
}