
use crate::{errors::ErrorCategory, utils};
use crate::rgb::RgbColor;
use crate::hsv::HsvColor;
use crate::hsl::HslColor;
use crate::errors;
use crate::errors::Result;


pub trait Conversion<TInput, TOutput>
{
    fn convert(&self, value : TInput) -> Result<TOutput>;
}

// create converter(struct)
#[allow(unused_macros)]
macro_rules! color_converter 
{
    ($converter_name: ident) => {
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]
        pub struct $converter_name{}

    };
}

// create converter(struct) + Conversion(trait)
#[allow(unused_macros)]
macro_rules! same_color_converter 
{
    ($converter_name: ident, $type_name: ident) => {
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]
        pub struct $converter_name{}

        impl Conversion<&$type_name,$type_name> for $converter_name
        {
            fn convert(&self, color: &$type_name) -> Result<$type_name>
            {
                Ok(color.clone())
            }
        }
    };
}

same_color_converter!(RgbToRgbConverter,RgbColor);
same_color_converter!(HsvToHsvConverter,HsvColor);
same_color_converter!(HslToHslConverter,HslColor);

color_converter!(RgbToHsvConverter);
color_converter!(RgbToHslConverter);
color_converter!(HsvToRgbConverter);
color_converter!(HsvToHslConverter);
color_converter!(HslToRgbConverter);
color_converter!(HslToHsvConverter);


impl Conversion<&RgbColor,HsvColor> for RgbToHsvConverter
{
    fn convert(&self, color: &RgbColor) -> Result<HsvColor>
    {
        let r = color.r();
        let g = color.g();
        let b = color.b();
        let max = utils::max(r, g, b);
        let min = utils::min(r, g, b);

        let v = (max as u16 * 100 / 255) as u8;
        let mut h = 0_i32; // TODO マイナス値を考慮するためi32
        let mut s = 0;

        if max != min{
            s = ((max - min) / max) * 100;

            if max == r
            {
                h = (g as i32 - b as i32) * 60 / (max - min) as i32;
            }
            else if max == g
            {
                h = 60 * (b as i32 - r as i32) / (max - min) as i32 + 120;
            }
            else if max == b
            {
                h = 60 *  (r as i32 - g as i32) / (max - min) as i32 + 240;
            }

            if h < 0{
                h += 360;
            }
        }

        Ok(HsvColor::from_hsva(h as u16, s, v, color.a()))
    }
}


impl Conversion<&RgbColor, HslColor> for RgbToHslConverter
{
    fn convert(&self, color: &RgbColor) -> Result<HslColor>
    {
        let r = color.r();
        let g = color.g();
        let b = color.b();
        let max = utils::max(r, g, b);
        let min = utils::min(r, g, b);

        let mut h = 0_i32; // TODO マイナス値を考慮するためi32
        let mut s = 0;
        let mut l = 0;

        if max != min{
            l = (((max as u16 + min as u16) as f64 / 2_f64) as f64 / 255_f64 * 100_f64) as u8;
            
            if max == r
            {
                h = ((g as i32 - b as i32) * 60 / (max - min) as i32);
            }
            else if max == g
            {
                h = 60 * (b as i32 - r as i32) / (max - min) as i32 + 120;
            }
            else if max == b
            {
                h = 60 * (r as i32 - g as i32) / (max - min) as i32 + 240;
            }

            if h < 0{
                h += 360;
            }

            let sum = max as u16 + min as u16;
            let sub = max as u16 - min as u16;
            let cnt = (max as u16 + min as u16) / 2;
            if cnt <= 127{
                s = ((sub as f64 / sum as f64) * 100_f64) as u8;
            }
            else{
                s = ((sub as f64 / (510_u16 - sum) as f64) * 100_f64) as u8;
            }

        }

        Ok(HslColor::from_hsla(h as u16, s, l as u8, color.a()))
    }

}


impl Conversion<&HsvColor, RgbColor> for HsvToRgbConverter
{
    fn convert(&self, color: &HsvColor) -> Result<RgbColor>
    {
        let h = if color.h() == 360 {0} else {color.h()};

        let hi = (((h as f64 / 60_f64).floor()) as u16) % 6;
        let f = (h as f64 / 60_f64) - hi as f64;
        let p = 255_f64 * (color.v() as f64 / 100_f64) * (1_f64 -  (color.s() as f64 / 100_f64));
        let q = 255_f64 * (color.v() as f64 / 100_f64) * (1_f64 - f * (color.s() as f64 / 100_f64));
        let t = 255_f64 * (color.v() as f64 / 100_f64) * (1_f64 - (1_f64 - f) * (color.s() as f64 /100_f64));
        let v = (color.v() as u16 * 255 / 100 ) as u8;

        match hi{
            0 => Ok(RgbColor::from_rgba(v, t as u8,  p as u8, color.a())),
            1 => Ok(RgbColor::from_rgba(q as u8,v,p as u8, color.a())),
            2 => Ok(RgbColor::from_rgba(p as u8,v,t as u8, color.a())),
            3 => Ok(RgbColor::from_rgba(p as u8,q as u8,v, color.a())),
            4 => Ok(RgbColor::from_rgba(t as u8,p as u8,v, color.a())),
            5 => Ok(RgbColor::from_rgba(v,p as u8,q as u8, color.a())),
            _ => Err(errors::Error::new(errors::ErrorCode::InvalidArgument, color.to_string()))
        }

    }


}

impl Conversion<&HsvColor, HslColor> for HsvToHslConverter
{
    fn convert(&self, color: &HsvColor) -> Result<HslColor>
    {
        // TODO Hsv => RGV => Hsl
        let rgb_converter = HsvToRgbConverter{};

        let result = rgb_converter.convert(color);
        if let Ok(rgb) = result
        {
            let hsl_converter = RgbToHslConverter{};
            hsl_converter.convert(&rgb)
        }
        else
        {
            Err(errors::Error::new(errors::ErrorCode::InvalidArgument, color.to_string()))
        }

    }

}

impl Conversion<&HslColor, RgbColor> for HslToRgbConverter
{
    #[allow(unused_assignments)]
    fn convert(&self, color: &HslColor) -> Result<RgbColor>
    {
        let mut max = 0_f64;
        let mut min = 0_f64;

        if color.l() < 50{
            max = (255 * (color.l() as u32 * 100 + (color.l() as u32 * color.s() as u32))) as f64 /10000_f64; 
            min = (255 * (color.l() as u32 * 100 - (color.l() as u32 * color.s() as u32))) as f64 /10000_f64; 
        }
        else{
            max = (255 * (color.l() as u32 * 100 + ((100 - color.l() as u32) * color.s() as u32)))as f64 / 10000_f64; 
            min = (255 * (color.l() as u32 * 100 - ((100 - color.l() as u32) * color.s() as u32)))as f64 / 10000_f64; 
        }

        let hi = (((color.h() as f64 / 60_f64).floor()) as u16) % 6;

        match hi{
            0 => Ok(RgbColor::from_rgba(max as u8, ((color.h() as f64 / 60_f64) * (max - min) + min) as u8, min as u8, color.a())),
            1 => Ok(RgbColor::from_rgba((((120 - color.h()) as f64 / 60_f64) * (max - min) + min) as u8,max as u8,min as u8, color.a())),
            2 => Ok(RgbColor::from_rgba(min as u8,max as u8, (((color.h() - 120) as f64 / 60_f64) * (max -min) + min) as u8, color.a())),
            3 => Ok(RgbColor::from_rgba(min as u8,(((240 - color.h()) as f64 / 60_f64) * (max - min) + min)as u8,max as u8, color.a())),
            4 => Ok(RgbColor::from_rgba((((color.h() - 240) as f64 / 60_f64) * (max - min) + min) as u8,min as u8,max as u8, color.a())),
            5 => Ok(RgbColor::from_rgba(max as u8,min as u8,(((360 - color.h()) as f64 / 60_f64) * (max -min) + min) as u8, color.a())),
            _ => Err(errors::Error::new(errors::ErrorCode::InvalidArgument, color.to_string()))
        }

    }
}

impl Conversion<&HslColor, HsvColor> for HslToHsvConverter
{
    fn convert(&self, color: &HslColor) -> Result<HsvColor>
    {
        // TODO Hsv => RGV => Hsl
        let rgb_converter = HslToRgbConverter{};
        let result = rgb_converter.convert(color);
        if let Ok(rgb) = result
        {
            let hsv_converter = RgbToHsvConverter{};
            hsv_converter.convert(&rgb)
        }
        else
        {
            Err(errors::Error::new(errors::ErrorCode::InvalidArgument, color.to_string()))
        }
    }

}


#[allow(dead_code)]
mod core
{
    const HEX_LOWER_TABLE : &[u8] = 
b"000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff";

    const HEX_UPPER_TABLE : &[u8] = 
b"000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F202122232425262728292A2B2C2D2E2F303132333435363738393A3B3C3D3E3F404142434445464748494A4B4C4D4E4F505152535455565758595A5B5C5D5E5F606162636465666768696A6B6C6D6E6F707172737475767778797A7B7C7D7E7F808182838485868788898A8B8C8D8E8F909192939495969798999A9B9C9D9E9FA0A1A2A3A4A5A6A7A8A9AAABACADAEAFB0B1B2B3B4B5B6B7B8B9BABBBCBDBEBFC0C1C2C3C4C5C6C7C8C9CACBCCCDCECFD0D1D2D3D4D5D6D7D8D9DADBDCDDDEDFE0E1E2E3E4E5E6E7E8E9EAEBECEDEEEFF0F1F2F3F4F5F6F7F8F9FAFBFCFDFEFF";
    pub(crate) const HASH_MARK_NUMBER :u8 = 35; // #

    pub(crate) fn decimal_to_lower_hexstring(value : u8, arr : &mut [u8])
    {
        let index : usize = (value as usize) << 1;
        arr[0] = HEX_LOWER_TABLE[index];
        arr[1] = HEX_LOWER_TABLE[index + 1];
    }

    pub(crate) fn decimal_to_upper_hexstring(value : u8, arr : &mut [u8])
    {
        let index : usize = (value as usize) << 1;
        arr[0] = HEX_UPPER_TABLE[index];
        arr[1] = HEX_UPPER_TABLE[index + 1];
    }

    pub(crate) fn hexstring_to_decimal(hexstring : &[u8]) -> u8
    {
        match hexstring.len()
        {
            1 => hex_char_to_decimal(hexstring[0]),
            2 => 16 * hex_char_to_decimal(hexstring[0]) + hex_char_to_decimal(hexstring[1]),
            _ => 0,
        }
    }

    fn hex_char_to_decimal(value :u8) -> u8
    {
        let v = value | 0x20u8;
        match v{
            48 ..= 57 => v - 48,
            97 ..= 102 => v - 87,
            _ => 0
        }
    }

    pub(crate) fn validate_hex_char(value: u8) -> bool
    {
        match value{
            48 ..= 57 => true,
            65 ..= 70 => true,
            97 ..= 102 => true,
            _ => false
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CharCase
{
    Lower,
    Upper,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct HtmlColorCode
{
    value: String
}

impl HtmlColorCode
{
    pub fn new(code: impl Into<String>) -> Self
    {
        Self{value: code.into()}
    }

    pub fn value(&self) -> String
    {
        self.value.to_string()
    }

    pub fn value_ref(&self) -> &str
    {
        &self.value
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct RgbToHtmlConverter
{}

impl Conversion<&RgbColor, HtmlColorCode> for RgbToHtmlConverter
{
    fn convert(&self, rgb : &RgbColor) -> Result<HtmlColorCode>
    {
        self.convert_with_charcase(&rgb, CharCase::Lower)
    }
}

impl RgbToHtmlConverter
{
    fn convert_with_charcase(&self, rgb: &RgbColor, charcase: CharCase) -> Result<HtmlColorCode>
    {
        match charcase
        {
            CharCase::Lower => Ok(self.to_html_lower(&rgb)),
            CharCase::Upper => Ok(self.to_html_upper(&rgb)),
        }
    }

    fn to_html_lower(&self, rgb : &RgbColor) -> HtmlColorCode
    {
        let mut html_array : [u8; 7] = Default::default();
        html_array[0] = core::HASH_MARK_NUMBER;
        core::decimal_to_lower_hexstring(rgb.r(), &mut html_array[1..3]);
        core::decimal_to_lower_hexstring(rgb.g(), &mut html_array[3..5]);
        core::decimal_to_lower_hexstring(rgb.b(), &mut html_array[5..7]);
        HtmlColorCode::new(html_array.iter().map(|&s| s as char).collect::<String>())
    }

    fn to_html_upper(&self, rgb : &RgbColor) -> HtmlColorCode
    {
        let mut html_array : [u8; 7] = Default::default();
        html_array[0] = core::HASH_MARK_NUMBER;
        core::decimal_to_upper_hexstring(rgb.r(), &mut html_array[1..3]);
        core::decimal_to_upper_hexstring(rgb.g(), &mut html_array[3..5]);
        core::decimal_to_upper_hexstring(rgb.b(), &mut html_array[5..7]);
        HtmlColorCode::new(html_array.iter().map(|&s| s as char).collect::<String>())
    }

}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct HsvToHtmlConverter
{}

impl Conversion<&HsvColor, HtmlColorCode> for HsvToHtmlConverter
{
    fn convert(&self, hsv : &HsvColor) -> Result<HtmlColorCode>
    {
        self.convert_with_charcase(&hsv, CharCase::Lower)
    }
}

impl HsvToHtmlConverter
{
    fn convert_with_charcase(&self, hsv: &HsvColor, charcase: CharCase) -> Result<HtmlColorCode>
    {
        let result = HsvToRgbConverter{}.convert(hsv);

        if let Ok(rgb) = result
        {
            RgbToHtmlConverter{}.convert_with_charcase(&rgb, charcase)
        }
        else
        {
            Err(errors::Error::new(errors::ErrorCode::InvalidArgument, hsv.to_string()))
        }
    }

}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct HslToHtmlConverter
{}

impl Conversion<&HslColor, HtmlColorCode> for HslToHtmlConverter
{
    fn convert(&self, hsl : &HslColor) -> Result<HtmlColorCode>
    {
        self.convert_with_charcase(&hsl, CharCase::Lower)
    }
}

impl HslToHtmlConverter
{
    fn convert_with_charcase(&self, hsl: &HslColor, charcase: CharCase) -> Result<HtmlColorCode>
    {
        let result = HslToRgbConverter{}.convert(hsl);

        if let Ok(rgb) = result
        {
            RgbToHtmlConverter{}.convert_with_charcase(&rgb, charcase)
        }
        else
        {
            Err(errors::Error::new(errors::ErrorCode::InvalidArgument, hsl.to_string()))
        }
    }

}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct HtmlToRgbConverter{}

impl Conversion<&HtmlColorCode, RgbColor> for HtmlToRgbConverter
{
    fn convert(&self, html : &HtmlColorCode) -> Result<RgbColor>
    {
        self.to_rgb(html)
    }
}

impl HtmlToRgbConverter
{
    fn to_rgb(&self, html :&HtmlColorCode) -> Result<RgbColor>
    {
        let s = html.value.as_bytes();

        if !self.validate_length(s)
        {
            return Err(errors::Error::new(errors::ErrorCode::InvalidArgument, html.value.to_string()))
        }

        if !self.validate_format(&s[1..])
        {
            return Err(errors::Error::new(errors::ErrorCode::InvalidArgumentFormat, html.value.to_string()))
        }

        let length = s.len();
        if s[0] == core::HASH_MARK_NUMBER
        {
            match length
            {
                7 => Ok(RgbColor::new(  
                                    core::hexstring_to_decimal(&s[1..=2]),
                                    core::hexstring_to_decimal(&s[3..=4]),
                                    core::hexstring_to_decimal(&s[5..=6]))),
                9 => Ok(RgbColor::from_rgba(  
                                    core::hexstring_to_decimal(&s[1..=2]),
                                    core::hexstring_to_decimal(&s[3..=4]),
                                    core::hexstring_to_decimal(&s[5..=6]),
                                    core::hexstring_to_decimal(&s[7..=8]))),
                _ => Err(errors::Error::new(errors::ErrorCode::InvalidArgument, html.value.to_string()))
            }
        }
        else
        {
            match length
            {
                6 => Ok(RgbColor::new(  
                                    core::hexstring_to_decimal(&s[0..=1]),
                                    core::hexstring_to_decimal(&s[2..=3]),
                                    core::hexstring_to_decimal(&s[4..=5]))),
                8 => Ok(RgbColor::from_rgba(  
                                    core::hexstring_to_decimal(&s[0..=1]),
                                    core::hexstring_to_decimal(&s[2..=3]),
                                    core::hexstring_to_decimal(&s[4..=5]),
                                    core::hexstring_to_decimal(&s[6..=7]))),
                _ => Err(errors::Error::new(errors::ErrorCode::InvalidArgument, html.value.to_string()))
            }
        }
    }

    fn validate_length(&self, raw_html :&[u8]) -> bool
    {
        match raw_html.len()
        {
            6 ..= 9 => true,
            _ => false
        }
    }

    fn validate_format(&self, raw_html: &[u8]) -> bool
    {
        !raw_html.iter().any(|&c| !core::validate_hex_char(c))
    }

}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct HtmlToHsvConverter{}

impl Conversion<&HtmlColorCode, HsvColor> for HtmlToHsvConverter
{
    fn convert(&self, html : &HtmlColorCode) -> Result<HsvColor>
    {
        let html_to_rgb_converter = HtmlToRgbConverter{};
        let rgb = html_to_rgb_converter.convert(html)?;

        let rgb_to_hsv_converter = RgbToHsvConverter{};
        rgb_to_hsv_converter.convert(&rgb)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct HtmlToHslConverter{}

impl Conversion<&HtmlColorCode, HslColor> for HtmlToHslConverter
{
    fn convert(&self, html : &HtmlColorCode) -> Result<HslColor>
    {
        let html_to_rgb_converter = HtmlToRgbConverter{};
        let rgb = html_to_rgb_converter.convert(html)?;

        let rgb_to_hsl_converter = RgbToHslConverter{};
        rgb_to_hsl_converter.convert(&rgb)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn rgb_to_hsv_converter_error_check_test() {
        for r in 0..=255
        {
            for g in 0..=255
            {
                for b in 0..=255
                {
                    let rgb = RgbColor::new(r, g, b);
                    let converter = RgbToHsvConverter{};
                    assert!(converter.convert(&rgb).is_ok());
                }
            }
        }
    }

    #[test]
    fn rgb_to_hsl_converter_error_check_test() {
        for r in 0..=255
        {
            for g in 0..=255
            {
                for b in 0..=255
                {
                    let rgb = RgbColor::new(r, g, b);
                    let converter = RgbToHslConverter{};
                    assert!(converter.convert(&rgb).is_ok());
                }
            }
        }
    }

    #[test]
    fn hsv_to_rgb_converter_error_check_test() {
        for h in 0..=360
        {
            for s in 0..=100
            {
                for v in 0..=100
                {
                    let hsv = HsvColor::new(h, s, v);
                    let converter = HsvToRgbConverter{};
                    assert!(converter.convert(&hsv).is_ok());
                }
            }
        }
    }

    #[test]
    fn hsv_to_hsl_converter_error_check_test() {
        for h in 0..=360
        {
            for s in 0..=100
            {
                for v in 0..=100
                {
                    let hsv = HsvColor::new(h, s, v);
                    let converter = HsvToHslConverter{};
                    assert!(converter.convert(&hsv).is_ok());
                }
            }
        }
    }


    #[test]
    fn hsl_to_rgb_converter_error_check_test() {
        for h in 0..=360
        {
            for s in 0..=100
            {
                for l in 0..=100
                {
                    let hsl = HslColor::new(h, s, l);
                    let converter = HslToRgbConverter{};
                    assert!(converter.convert(&hsl).is_ok());
                }
            }
        }
    }

    #[test]
    fn hsl_to_hsv_converter_error_check_test() {
        for h in 0..=360
        {
            for s in 0..=100
            {
                for l in 0..=100
                {
                    let hsl = HslColor::new(h, s, l);
                    let converter = HslToHsvConverter{};
                    assert!(converter.convert(&hsl).is_ok());
                }
            }
        }
    }

    #[test]
    fn rgb_to_hsv_converter_convert_test() {
        
        let converter = RgbToHsvConverter{};
        let hsv = converter.convert(&RgbColor::new(255, 0, 0)).unwrap();
        assert_eq!(hsv, HsvColor::new(0, 100, 100));

    }

    #[test]
    fn rgb_to_hsv_converter_convert_test2() {
        
        let converter = RgbToHsvConverter{};
        let hsv = converter.convert(&RgbColor::new(0, 255, 0)).unwrap();
        assert_eq!(hsv, HsvColor::new(120, 100, 100));

    }

    #[test]
    fn rgb_to_hsv_converter_convert_test3() {
        
        let converter = RgbToHsvConverter{};
        let hsv = converter.convert(&RgbColor::new(0, 0, 255)).unwrap();
        assert_eq!(hsv, HsvColor::new(240, 100, 100));

    }

    #[test]
    fn rgb_to_hsl_converter_convert_test() {
        
        let converter = RgbToHslConverter{};
        let hsl = converter.convert(&RgbColor::new(255, 0, 0)).unwrap();
        assert_eq!(hsl, HslColor::new(0, 100, 50));

    }

    #[test]
    fn rgb_to_hsl_converter_convert_test2() {
        
        let converter = RgbToHslConverter{};
        let hsl = converter.convert(&RgbColor::new(0, 255, 0)).unwrap();
        assert_eq!(hsl, HslColor::new(120, 100, 50));

    }

    #[test]
    fn rgb_to_hsl_converter_convert_test3() {
        
        let converter = RgbToHslConverter{};
        let hsl = converter.convert(&RgbColor::new(0, 0, 255)).unwrap();
        assert_eq!(hsl, HslColor::new(240, 100, 50));

    }

    #[test]
    fn hsv_to_rgb_converter_convert_test() {
        
        let converter = HsvToRgbConverter{};
        let rgb = converter.convert(&HsvColor::new(0, 100, 100)).unwrap();
        assert_eq!(rgb, RgbColor::new(255, 0, 0));

    }

    #[test]
    fn hsv_to_rgb_converter_convert_test2() {
        
        let converter = HsvToRgbConverter{};
        let rgb = converter.convert(&HsvColor::new(120, 100, 100)).unwrap();
        assert_eq!(rgb, RgbColor::new(0, 255, 0));

    }

    #[test]
    fn hsv_to_rgb_converter_convert_test3() {
        
        let converter = HsvToRgbConverter{};
        let rgb = converter.convert(&HsvColor::new(240, 100, 100)).unwrap();
        assert_eq!(rgb, RgbColor::new(0, 0, 255));

    }

    #[test]
    fn hsv_to_hsl_converter_convert_test() {
        
        let converter = HsvToHslConverter{};
        let hsl = converter.convert(&HsvColor::new(0, 100, 100)).unwrap();
        assert_eq!(hsl, HslColor::new(0, 100, 50));
    }

    #[test]
    fn hsv_to_hsl_converter_convert_test2() {
        
        let converter = HsvToHslConverter{};
        let hsl = converter.convert(&HsvColor::new(120, 100, 100)).unwrap();
        assert_eq!(hsl, HslColor::new(120, 100, 50));
    }

    #[test]
    fn hsv_to_hsl_converter_convert_test3() {
        
        let converter = HsvToHslConverter{};
        let hsl = converter.convert(&HsvColor::new(240, 100, 100)).unwrap();
        assert_eq!(hsl, HslColor::new(240, 100, 50));
    }


    #[test]
    fn hsl_to_rgb_converter_convert_test() {
        
        let converter = HslToRgbConverter{};
        let rgb = converter.convert(&HslColor::new(0, 100, 50)).unwrap();
        assert_eq!(rgb, RgbColor::new(255, 0, 0));

    }

    #[test]
    fn hsl_to_rgb_converter_convert_test2() {
        
        let converter = HslToRgbConverter{};
        let rgb = converter.convert(&HslColor::new(120, 100, 50)).unwrap();
        assert_eq!(rgb, RgbColor::new(0, 255, 0));

    }

    #[test]
    fn hsl_to_rgb_converter_convert_test3() {
        
        let converter = HslToRgbConverter{};
        let rgb = converter.convert(&HslColor::new(240, 100, 50)).unwrap();
        assert_eq!(rgb, RgbColor::new(0, 0, 255));

    }

    #[test]
    fn hsl_to_hsv_converter_convert_test() {
        
        let converter = HslToHsvConverter{};
        let hsv = converter.convert(&HslColor::new(0, 100, 50)).unwrap();
        assert_eq!(hsv, HsvColor::new(0, 100, 100));

    }

    #[test]
    fn hsl_to_hsv_converter_convert_test2() {
        
        let converter = HslToHsvConverter{};
        let hsv = converter.convert(&HslColor::new(120, 100, 50)).unwrap();
        assert_eq!(hsv, HsvColor::new(120, 100, 100));

    }

    #[test]
    fn hsl_to_hsv_converter_convert_test3() {
        
        let converter = HslToHsvConverter{};
        let hsv = converter.convert(&HslColor::new(240, 100, 50)).unwrap();
        assert_eq!(hsv, HsvColor::new(240, 100, 100));

    }


    #[test]
    fn to_html_lower_new_test() {
        let rgb = RgbColor::new(255, 0, 0);
        let html = RgbToHtmlConverter{}.convert(&rgb).unwrap();

        assert_eq!(html, HtmlColorCode::new("#ff0000"));
    }

    #[test]
    fn to_html_lower_new_test2() {
        let rgb = RgbColor::new(124, 53, 234);
        let html = RgbToHtmlConverter{}.convert(&rgb).unwrap();

        assert_eq!(html, HtmlColorCode::new("#7c35ea"));
    }

    #[test]
    fn to_html_upper_new_test() {
        let rgb = RgbColor::new(255, 0, 0);
        let html = RgbToHtmlConverter{}.convert_with_charcase(&rgb, CharCase::Upper).unwrap();

        assert_eq!(html, HtmlColorCode::new("#FF0000"));
    }


    #[test]
    fn to_html_upper_new_test2() {
        let rgb = RgbColor::new(124, 53, 234);
        let html = RgbToHtmlConverter{}.convert_with_charcase(&rgb, CharCase::Upper).unwrap();
        assert_eq!(html, HtmlColorCode::new("#7C35EA"));
    }

    #[test]
    fn html_to_rgb_new_test() {
        let html = HtmlColorCode::new("#FFFFFF");
        let rgb = HtmlToRgbConverter{}.convert(&html).unwrap();
        assert_eq!(rgb,RgbColor::new(255, 255, 255));
    }

    #[test]
    fn html_to_rgb_new_test2() {
        let html = HtmlColorCode::new("#FFFFFFFF");
        let rgb = HtmlToRgbConverter{}.convert(&html).unwrap();
        assert_eq!(rgb,RgbColor::from_rgba(255, 255, 255, 255));
    }


    #[test]
    fn html_to_rgb_new_test3() {
        let html = HtmlColorCode::new("FFFFFF");
        let rgb = HtmlToRgbConverter{}.convert(&html).unwrap();
        assert_eq!(rgb,RgbColor::new(255, 255, 255));
        
    }


    #[test]
    fn html_to_rgb_new_test4() {
        let html = HtmlColorCode::new("FFFFFFFF");
        let rgb = HtmlToRgbConverter{}.convert(&html).unwrap();
        assert_eq!(rgb,RgbColor::from_rgba(255, 255, 255, 255));
        
    }

    #[test]
    fn html_to_rgb_new_test_format_err() {
        let html = HtmlColorCode::new("FFBFsF");
        let rgb = HtmlToRgbConverter{}.convert(&html);
        assert!(rgb.is_err());
    }


    #[test]
    fn html_to_rgb_new_test_format_err2() {
        let html = HtmlColorCode::new("#fsadfg");
        let rgb = HtmlToRgbConverter{}.convert(&html);
        assert!(rgb.is_err());
    }

    #[test]
    fn html_to_rgb_new_test_length_err() {
        let html = HtmlColorCode::new("989898898989");
        let rgb = HtmlToRgbConverter{}.convert(&html);
        assert!(rgb.is_err());
    }

    #[test]
    fn html_to_rgb_new_test_length_err2() {
        let html = HtmlColorCode::new("98988");
        let rgb = HtmlToRgbConverter{}.convert(&html);
        assert!(rgb.is_err());
    }

    #[test]
    fn html_to_rgb_new_test_err3() {
        let html = HtmlColorCode::new("あいうえお");
        let rgb = HtmlToRgbConverter{}.convert(&html);
        assert!(rgb.is_err());
    }

    #[test]
    fn html_core_test() {
        assert_eq!(core::hexstring_to_decimal(b"0"),0);
        assert_eq!(core::hexstring_to_decimal(b"4"),4);
        assert_eq!(core::hexstring_to_decimal(b"9"),9);
        assert_eq!(core::hexstring_to_decimal(b"00"),0);
        assert_eq!(core::hexstring_to_decimal(b"09"),9);
        assert_eq!(core::hexstring_to_decimal(b"4e"),78);
        assert_eq!(core::hexstring_to_decimal(b"AC"),172);
        assert_eq!(core::hexstring_to_decimal(b"FF"),255);
        assert_eq!(core::hexstring_to_decimal(b"fsd"),0);
        assert_eq!(core::hexstring_to_decimal(b"9ss"),0);
    }
}
