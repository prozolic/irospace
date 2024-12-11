use std::fmt::Display;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct RgbColor {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Display for RgbColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RgbColor R = {} G = {} B = {} A = {}",
            self.r, self.g, self.b, self.a
        )
    }
}

impl RgbColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self::from_rgba(r, g, b, 255)
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }

    pub fn a(&self) -> u8 {
        self.a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_new_test() {
        let red = RgbColor::new(255, 0, 0);

        assert_eq!(red.r(), 255);
        assert_eq!(red.g(), 0);
        assert_eq!(red.b(), 0);
        assert_eq!(red.a(), 255);
    }
}
