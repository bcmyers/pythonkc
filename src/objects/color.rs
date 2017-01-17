#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Color {
    blue: u8,
    green: u8,
    red: u8,
}

impl Color {
    pub fn new(blue: u8, green: u8, red: u8) -> Color {
        Color {
            blue: blue,
            green: green,
            red: red,
        }
    }
}
