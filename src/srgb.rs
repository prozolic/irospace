use std::fmt::Display;

#[allow(non_camel_case_types)]
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct sRgbColor {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

impl Display for sRgbColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "sRgbColor R = {} G = {} B = {} A = {}",
            self.r, self.g, self.b, self.a
        )
    }
}

impl sRgbColor {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self::from_srgba(r, g, b, 1f64)
    }

    pub fn from_srgba(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn g(&self) -> f64 {
        self.g
    }

    pub fn b(&self) -> f64 {
        self.b
    }

    pub fn a(&self) -> f64 {
        self.a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn srgb_new_test() {
        let red = sRgbColor::new(1f64, 0f64, 0f64);

        assert_eq!(red.r(), 1f64);
        assert_eq!(red.g(), 0f64);
        assert_eq!(red.b(), 0f64);
        assert_eq!(red.a(), 1f64);
    }
}
