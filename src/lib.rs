mod utils;

pub mod colors;
pub mod hsl;
pub mod hsv;
pub mod rgb;
pub mod srgb;

pub mod converter;
pub mod converter_builder;
pub mod errors;

pub use colors::Colors;
pub use converter::{HslToHslConverter, HslToHsvConverter, HslToRgbConverter};
pub use converter::{HsvToHslConverter, HsvToHsvConverter, HsvToRgbConverter};
pub use converter::{RgbToHslConverter, RgbToHsvConverter, RgbToHtmlConverter, RgbToRgbConverter};
pub use converter_builder::{
    ColorConverterBuilder, ColorConverterFromBuilder, ColorConverterFromToBuilder,
};
pub use errors::{Error, ErrorCategory};
pub use hsl::HslColor;
pub use hsv::HsvColor;
pub use rgb::RgbColor;
