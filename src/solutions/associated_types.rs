const PI: f64 = 3.14;

pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

pub struct Circle {
    pub radius: f64,
}

pub trait Shape {
    type Unit;

    fn area(&self) -> Self::Unit;
}

impl Shape for Rectangle {
    type Unit = u32;
    fn area(&self) -> Self::Unit {
        self.width + self.height
    }
}

impl Shape for Circle {
    type Unit = f64;
    fn area(&self) -> Self::Unit {
        PI * (self.radius).powi(2)
    }
}
