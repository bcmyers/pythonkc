use std::f64::consts::PI;

use super::color::Color;
use super::traits::Shape;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Circle {
    color: Color,
    radius: f64,
}

impl Circle {
    pub fn new<T: Into<f64> + Copy>(radius: T) -> Circle {
        Circle {
            color: Color::new(255, 255, 255),
            radius: radius.into(),
        }
    }
    pub fn get_circumference(&self) -> f64 {
        2f64 * PI * self.radius
    }
    pub fn set_circumference<T: Into<f64> + Copy>(&mut self, circumference: T) {
        self.radius = circumference.into() / (2f64 * PI)
    }
    pub fn get_diameter(&self) -> f64 {
        2f64 * self.radius
    }
    pub fn set_diamenter<T: Into<f64> + Copy>(&mut self, diameter: T) {
        self.radius = (diameter.into() / PI).sqrt();
    }
    pub fn get_radius(&self) -> f64 {
        self.radius
    }
    pub fn set_radius<T: Into<f64> + Copy>(&mut self, radius: T) {
        self.radius = radius.into();
    }
}

impl Shape for Circle {
    fn get_area(&self) -> f64 {
        PI * self.radius.powi(2)
    }
    fn set_area<T: Into<f64> + Copy>(&mut self, area: T) {
        self.radius = (area.into() / PI).sqrt();
    }
    fn get_color(&self) -> Color {
        self.color
    }
    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let circle = Circle::new(5u8);
        assert_eq!(circle.get_area(), PI * 5f64.powi(2));
        assert_eq!(circle.get_color(), Color::new(255, 255, 255));
        assert_eq!(circle.get_circumference(), 2f64 * PI * 5f64);
        assert_eq!(circle.get_diameter(), 2f64 * 5f64);
        assert_eq!(circle.get_radius(), 5f64);
    }
}
