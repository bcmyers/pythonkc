use super::color::Color;
use super::traits::Shape;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Square {
    area: f64,
    color: Color,
    side: f64,
}

impl Square {
    pub fn new<T: Into<f64> + Copy>(side: T) -> Square {
        Square {
            area: side.into().powi(2),
            color: Color::new(255, 255, 255),
            side: side.into(),
        }
    }
    pub fn get_side(&self) -> f64 {
        self.side
    }
    pub fn set_side<T: Into<f64> + Copy>(&mut self, side: T) {
        self.area = side.into().powi(2);
        self.side = side.into();
    }
}

impl Shape for Square {
    fn get_area(&self) -> f64 {
        self.side.powi(2)
    }
    fn set_area<T: Into<f64> + Copy>(&mut self, area: T) {
        self.area = area.into();
        self.side = area.into().sqrt();
    }
    fn get_color(&self) -> Color {
        self.color
    }
    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}
