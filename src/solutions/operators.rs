use std::fmt::Display;
use std::ops::Add;

pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, point: Self) -> Self {
        Self {
            x: self.x + point.x,
            y: self.y + point.y,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point ({}, {})", self.x, self.y)
    }
}
