
use std::fmt::Display;

use crate::utils;


#[derive(Debug,Default,PartialEq,Eq,Clone,Copy)]
pub struct HslColor
{
    h : u16,
    s : u8,
    l : u8,
    a : u8,
}

impl Display for HslColor
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HslColor H = {} S = {} L = {} A = {}",self.h, self.s, self.l ,self.a)
    }
}

impl HslColor
{
    pub fn new(h : u16, s : u8, l : u8) -> Self
    {
        Self::from_hsla(h, s, l, 255)
    }

    pub fn from_hsla(h : u16, s : u8, l : u8, a : u8) -> Self
    {
        Self
        {
            h : utils::crop_range(h,0,360), 
            s : utils::crop_range(s,0,100), 
            l : utils::crop_range(l,0,100), 
            a,
        }
    }

    pub fn h(&self) -> u16
    {
        self.h
    }

    pub fn s(&self) -> u8
    {
        self.s
    }

    pub fn l(&self) -> u8
    {
        self.l
    }

    pub fn a(&self) -> u8
    {
        self.a
    }



}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn hsl_new_test() {
        let red = HslColor::new(0,100,100);

        assert_eq!(red.h(), 0);
        assert_eq!(red.s(), 100);
        assert_eq!(red.l(), 100);
        assert_eq!(red.a(), 255);
    }


}
