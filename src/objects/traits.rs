use super::color::Color;

pub trait Shape {
    fn area(&self) -> f64;
    fn color(&self) -> Color;
    fn set_area<T: Into<f64> + Copy>(&mut self, area: T);
    fn set_color(&mut self, color: Color);
}
