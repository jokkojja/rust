pub trait Convertable<T> {
    fn convert(&self) -> T;
}

pub struct Celsius(pub f32);
pub struct Fahrenheit(pub f32);

impl Convertable<Fahrenheit> for Celsius {
    fn convert(&self) -> Fahrenheit {
        Fahrenheit(self.0 * 1.8 + 32.0)
    }
}

impl Convertable<Celsius> for Fahrenheit {
    fn convert(&self) -> Celsius {
        Celsius((self.0 - 32.0) / 1.8)
    }
}
