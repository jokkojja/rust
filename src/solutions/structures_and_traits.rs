pub mod structures_and_traits {
    pub const PI: f32 = 3.14;

    pub trait Drawable<T> {
        fn draw(&self, figure: &T);
    }

    pub struct Drower;

    impl Drawable<Circle> for Drower {
        fn draw(&self, figure: &Circle) {
            println!("Рисую круг: radius - {}", figure.radius);
        }
    }

    impl Drawable<Rectangle> for Drower {
        fn draw(&self, figure: &Rectangle) {
            println!("Рисую прямоугольник: a - {}, b - {}", figure.a, figure.b);
        }
    }

    pub struct Circle {
        pub radius: i16,
    }

    pub struct Rectangle {
        pub a: i16,
        pub b: i16,
    }
}
