use std::{fmt, hash::{Hash, Hasher}, ops::{Add, AddAssign, Sub}};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for Vector {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);  // Hash the `x` field
        self.y.hash(state);  // Hash the `y` field
    }
}

impl Eq for Vector {}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Self) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Self) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}