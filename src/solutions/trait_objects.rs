use std::fmt::Display;

pub struct Circle {
    pub radius: f32,
}

pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

pub trait Shape {
    fn area(&self) -> f32;
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        (self.radius).powi(2) * 3.14
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle: {}", { self.radius })
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rectangle: height - {}, width - {}",
            self.height, self.width
        )
    }
}
impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.width + self.height
    }
}
