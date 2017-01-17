use super::color::Color;

pub trait Shape {
    fn get_area(&self) -> f64;
    fn set_area<T: Into<f64> + Copy>(&mut self, area: T);
    fn get_color(&self) -> Color;
    fn set_color(&mut self, color: Color);
}
