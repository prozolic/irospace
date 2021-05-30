
use std::{marker::PhantomData};

use crate::rgb::RgbColor;
use crate::hsv::HsvColor;
use crate::hsl::HslColor;
use crate::converter::*;


pub struct ColorConverterBuilder{}
pub struct ColorConverterFromBuilder<TFrom>( PhantomData<TFrom> );
pub struct ColorConverterFromToBuilder<TFrom, TTo>( PhantomData<TFrom>, PhantomData<TTo>);

pub struct HtmlConverterFromBuilder{}

pub struct HtmlConverterFromToBuilder<TTo>(PhantomData<TTo> );


impl ColorConverterBuilder
{
    pub fn new() -> Self
    {
        ColorConverterBuilder{}
    }

    pub fn from_rgb(&self) -> ColorConverterFromBuilder<RgbColor>
    {
        ColorConverterFromBuilder(PhantomData)
    }

    pub fn from_hsv(&self) -> ColorConverterFromBuilder<HsvColor>
    {
        ColorConverterFromBuilder(PhantomData)
    }

    pub fn from_hsl(&self) -> ColorConverterFromBuilder<HslColor>
    {
        ColorConverterFromBuilder(PhantomData)
    }

    pub fn from_html(&self) -> HtmlConverterFromBuilder
    {
        HtmlConverterFromBuilder{}
    }
}

impl HtmlConverterFromBuilder
{
    pub fn to_rgb(&self) -> ColorConverterFromToBuilder<HtmlColorCode, RgbColor>
    {
        ColorConverterFromToBuilder(PhantomData,PhantomData)
    }

    pub fn to_hsv(&self) -> ColorConverterFromToBuilder<HtmlColorCode, HsvColor>
    {
        ColorConverterFromToBuilder(PhantomData,PhantomData)
    }

    pub fn to_hsl(&self) -> ColorConverterFromToBuilder<HtmlColorCode, HslColor>
    {
        ColorConverterFromToBuilder(PhantomData,PhantomData)
    }
}

#[allow(unused_macros)]
macro_rules! color_from_builder 
{
    ($from_name: ident) => {
        impl ColorConverterFromBuilder<$from_name>
        {
            pub fn to_rgb(&self) -> ColorConverterFromToBuilder<$from_name, RgbColor>
            {
                ColorConverterFromToBuilder(PhantomData,PhantomData)
            }
        
            pub fn to_hsv(&self) -> ColorConverterFromToBuilder<$from_name, HsvColor>
            {
                ColorConverterFromToBuilder(PhantomData,PhantomData)
            }
        
            pub fn to_hsl(&self) -> ColorConverterFromToBuilder<$from_name, HslColor>
            {
                ColorConverterFromToBuilder(PhantomData,PhantomData)
            }

            pub fn to_html(&self) -> ColorConverterFromToBuilder<$from_name, HtmlColorCode>
            {
                ColorConverterFromToBuilder(PhantomData,PhantomData)
            }
        }        
    };
}

#[allow(unused_macros)]
macro_rules! color_from_to_builder 
{
    ($converter_name: ident, $from_name: ident, $to_name: ident) => {
        impl ColorConverterFromToBuilder<$from_name, $to_name>
        {
            #[allow(dead_code)]
            pub fn build(&self) -> $converter_name
            {
                $converter_name{}
            }
        }
    };
}

color_from_builder!(RgbColor);
color_from_builder!(HsvColor);
color_from_builder!(HslColor);

color_from_to_builder!(RgbToRgbConverter,RgbColor,RgbColor);
color_from_to_builder!(RgbToHsvConverter,RgbColor,HsvColor);
color_from_to_builder!(RgbToHslConverter,RgbColor,HslColor);

color_from_to_builder!(HsvToRgbConverter,HsvColor,RgbColor);
color_from_to_builder!(HsvToHsvConverter,HsvColor,HsvColor);
color_from_to_builder!(HsvToHslConverter,HsvColor,HslColor);

color_from_to_builder!(HslToRgbConverter,HslColor,RgbColor);
color_from_to_builder!(HslToHsvConverter,HslColor,HsvColor);
color_from_to_builder!(HslToHslConverter,HslColor,HslColor);

color_from_to_builder!(HtmlToRgbConverter,HtmlColorCode,RgbColor);
color_from_to_builder!(HtmlToHsvConverter,HtmlColorCode,HsvColor);
color_from_to_builder!(HtmlToHslConverter,HtmlColorCode,HslColor);

color_from_to_builder!(RgbToHtmlConverter,RgbColor,HtmlColorCode);
color_from_to_builder!(HslToHtmlConverter,HsvColor,HtmlColorCode);
color_from_to_builder!(HsvToHtmlConverter,HslColor,HtmlColorCode);


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn color_build_new_test() {
        let converter = ColorConverterBuilder::new().from_rgb().to_hsv().build();
        assert_eq!(converter.convert(&RgbColor::new(255,0,0)).unwrap(), HsvColor::new(0,100,100));

    }

    #[test]
    fn color_build_new_test2() {
        let converter = ColorConverterBuilder::new().from_rgb().to_html().build();
        assert_eq!(converter.convert(&RgbColor::new(255,0,0)).unwrap(), HtmlColorCode::new("#ff0000"));

    }

    #[test]
    fn color_build_new_test3() {
        let converter = ColorConverterBuilder::new().from_html().to_rgb().build();
        assert_eq!(converter.convert(&HtmlColorCode::new("#ff0000")).unwrap(), RgbColor::new(255,0,0));

    }
}
