use std::fmt::Display;

use crate::utils;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct HsvColor {
    h: u16,
    s: u8,
    v: u8,
    a: u8,
}

impl Display for HsvColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HsvColor H = {} S = {} V = {} A = {}",
            self.h, self.s, self.v, self.a
        )
    }
}

impl HsvColor {
    pub fn new(h: u16, s: u8, v: u8) -> Self {
        Self::from_hsva(h, s, v, 255)
    }

    pub fn from_hsva(h: u16, s: u8, v: u8, a: u8) -> Self {
        Self {
            h: utils::crop_range(h, 0, 360),
            s: utils::crop_range(s, 0, 100),
            v: utils::crop_range(v, 0, 100),
            a,
        }
    }

    pub fn h(&self) -> u16 {
        self.h
    }

    pub fn s(&self) -> u8 {
        self.s
    }

    pub fn v(&self) -> u8 {
        self.v
    }

    pub fn a(&self) -> u8 {
        self.a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hsv_new_test() {
        let red = HsvColor::new(0, 100, 100);

        assert_eq!(red.h(), 0);
        assert_eq!(red.s(), 100);
        assert_eq!(red.v(), 100);
    }
}
