
use crate::rgb::RgbColor;
use crate::hsv::HsvColor;
use crate::hsl::HslColor;


macro_rules! colors_iterable_enum {
    ($visibility:vis, $name:ident, $($value:tt),*) => {
        #[derive(Debug,Clone,Copy)]
        $visibility enum $name {$($value),*}
        impl $name 
        {
            fn iter() -> impl Iterator<Item = $name> 
            {
                [$(Self::$value,)*].iter().copied()
            }
        }

        impl std::fmt::Display for $name
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
            {
                match self
                {
                    $(Self::$value => write!(f,"{:?}",Self::$value),)*
                }
            }
        }
    };
    ($name:ident, $($value:tt),*) => {
        iterable_enum!(, $name, $($value),*)
    };
}

colors_iterable_enum!(
    pub, Colors, 
    Aqua,
    Black,
    Blue,	
    Fuchsia,	
    Gray,	
    Green,	
    Lime,	
    Maroon,	
    Navy,	
    Olive,
    Orange,	
    Purple,	
    Red,	
    Silver,	
    Teal,	
    White,	
    Yellow
);

impl Colors
{
    // define base colors

    pub fn to_rgb(&self) -> RgbColor
    { 
        match self
        {
            Self::Aqua => RgbColor::new(0, 0xFF, 0xFF),
            Self::Black => RgbColor::new(0, 0, 0),
            Self::Blue => RgbColor::new(0, 0, 0xFF),	
            Self::Fuchsia => RgbColor::new(0xFF, 0, 0xFF),	
            Self::Gray => RgbColor::new(0x80, 0x80, 0x80),	
            Self::Green => RgbColor::new(0, 0x80, 0),	
            Self::Lime => RgbColor::new(0, 0xFF, 0),	
            Self::Maroon => RgbColor::new(0x80, 0, 0),	
            Self::Navy => RgbColor::new(0, 0, 0x80),	
            Self::Olive => RgbColor::new(0x80, 0x80, 0),
            Self::Orange => RgbColor::new(0xFF, 0xA5, 0),	
            Self::Purple => RgbColor::new(0x80, 0, 0x80),	
            Self::Red => RgbColor::new(0xFF, 0, 0),	
            Self::Silver => RgbColor::new(0xc0, 0xc0, 0xc0),	
            Self::Teal => RgbColor::new(0, 0x80, 0x80),	
            Self::White => RgbColor::new(0xFF, 0xFF, 0xFF),	
            Self::Yellow => RgbColor::new(0xFF, 0xFF, 0),
        }
    }

    pub fn to_hsv(&self) -> HsvColor
    {
        match self
        {
            Self::Aqua => HsvColor::new(180, 100, 100),
            Self::Black => HsvColor::new(0, 0, 0),
            Self::Blue => HsvColor::new(240, 100, 100),	
            Self::Fuchsia => HsvColor::new(300, 100, 100),	
            Self::Gray => HsvColor::new(0, 0, 50),	
            Self::Green => HsvColor::new(120, 100, 50),	
            Self::Lime => HsvColor::new(120, 100, 100),	
            Self::Maroon => HsvColor::new(0, 100, 50),	
            Self::Navy => HsvColor::new(240, 100, 50),	
            Self::Olive => HsvColor::new(60, 100, 50),
            Self::Orange => HsvColor::new(38, 100, 100),	
            Self::Purple => HsvColor::new(300, 100, 50),	
            Self::Red => HsvColor::new(0, 100, 100),	
            Self::Silver => HsvColor::new(0, 0, 75),	
            Self::Teal => HsvColor::new(180, 100, 50),	
            Self::White => HsvColor::new(0, 0, 100),	
            Self::Yellow => HsvColor::new(60, 100, 100),
        }
    }

    pub fn to_hsl(&self) -> HslColor
    {
        match self
        {
            Self::Aqua => HslColor::new(180, 100, 50),
            Self::Black => HslColor::new(0, 0, 0),
            Self::Blue => HslColor::new(240, 100, 50),	
            Self::Fuchsia => HslColor::new(300, 100, 50),	
            Self::Gray => HslColor::new(0, 0, 50),	
            Self::Green => HslColor::new(120, 100, 25),	
            Self::Lime => HslColor::new(120, 100, 50),	
            Self::Maroon => HslColor::new(0, 100, 25),	
            Self::Navy => HslColor::new(240, 100, 25),	
            Self::Olive => HslColor::new(60, 100, 25),
            Self::Orange => HslColor::new(38, 100, 50),		
            Self::Purple => HslColor::new(300, 100, 25),	
            Self::Red => HslColor::new(0, 100, 50),	
            Self::Silver => HslColor::new(0, 0, 75),	
            Self::Teal => HslColor::new(180, 100, 25),	
            Self::White => HslColor::new(0, 0, 100),	
            Self::Yellow => HslColor::new(60, 100, 50),
        }
    }
    
}


impl Colors
{
    pub fn total_number() -> usize
    {
        Self::iter().count()
    }




}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn colors_name_test() 
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
    fn colors_iter_test() 
    {
        for color in Colors::iter()
        {
            println!("color name: {}",color);
        }
    }



}


