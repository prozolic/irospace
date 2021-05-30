
extern crate irospace;
use irospace::{colors::Colors,
    RgbColor,HsvColor,HslColor,
    converter::*,
    ColorConverterBuilder};

#[test]
fn colors_test()
{
    assert_eq!(Colors::Aqua.to_string(),"Aqua");
    assert_eq!(Colors::Black.to_string(),"Black");
    assert_eq!(Colors::Blue.to_string(),"Blue");	
    assert_eq!(Colors::Fuchsia.to_string(),"Fuchsia");	
    assert_eq!(Colors::Gray.to_string(),"Gray");	
    assert_eq!(Colors::Green.to_string(),"Green");	
    assert_eq!(Colors::Lime.to_string(),"Lime");	
    assert_eq!(Colors::Maroon.to_string(),"Maroon");	
    assert_eq!(Colors::Navy.to_string(),"Navy");	
    assert_eq!(Colors::Olive.to_string(),"Olive");	
    assert_eq!(Colors::Purple.to_string(),"Purple");	
    assert_eq!(Colors::Red.to_string(),"Red");	
    assert_eq!(Colors::Silver.to_string(),"Silver");	
    assert_eq!(Colors::Teal.to_string(),"Teal");	
    assert_eq!(Colors::White.to_string(),"White");	
    assert_eq!(Colors::Yellow.to_string(),"Yellow");  
}


#[test]
fn common_test_from_rgb_to_hsv()
{
    let converter = ColorConverterBuilder::new().from_rgb().to_hsv().build();
    assert_eq!(converter.convert(&Colors::Red.to_rgb()).unwrap(), Colors::Red.to_hsv()); 
}

#[test]
fn common_test_from_rgb_to_hsl()
{
    let converter = ColorConverterBuilder::new().from_rgb().to_hsl().build();
    assert_eq!(converter.convert(&Colors::Red.to_rgb()).unwrap(), Colors::Red.to_hsl()); 
}

#[test]
fn common_test_from_hsv_to_rgb()
{
    let converter = ColorConverterBuilder::new().from_hsv().to_rgb().build();
    assert_eq!(converter.convert(&Colors::Red.to_hsv()).unwrap(), Colors::Red.to_rgb()); 
}

#[test]
fn common_test_from_hsv_to_hsl()
{
    let converter = ColorConverterBuilder::new().from_hsv().to_hsl().build();
    assert_eq!(converter.convert(&Colors::Red.to_hsv()).unwrap(), Colors::Red.to_hsl()); 
}

#[test]
fn common_test_from_hsl_to_rgb()
{
    let converter = ColorConverterBuilder::new().from_hsl().to_rgb().build();
    assert_eq!(converter.convert(&Colors::Red.to_hsl()).unwrap(), Colors::Red.to_rgb()); 
}

#[test]
fn common_test_from_hsl_to_hsv()
{
    let converter = ColorConverterBuilder::new().from_hsl().to_hsv().build();
    assert_eq!(converter.convert(&Colors::Red.to_hsl()).unwrap(), Colors::Red.to_hsv()); 
}