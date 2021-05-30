# irospace

[![MIT License](http://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)

## 概要
irospaceは、単純な色空間や色空間変換処理を提供するライブラリです。

### 機能
* 色空間変換  
* カラーコード(HTMLカラーコード)変換
* システムカラー(17色)を定義

## サポート色空間
+ RGB(RGBA)
+ HSV
+ HSL

## サポートカラー(17色)
* Aqua
* Black
* Blue,
* Fuchsia
* Gray
* Green
* Lime
* Maroon	
* Navy
* Olive
* Orange	
* Purple	
* Red	
* Silver	
* Teal
* White	
* Yellow

## サンプル
### RGB -> HSV or HSL
```rust
extern crate irospace;
use irospace::{colors::Colors,
    RgbColor,HsvColor,HslColor,
    converter::*,
    ColorConverterBuilder};

fn from_rgb_to_hsv()
{
    let rgb = RgbColor::new(255,0,0);
    let converter = ColorConverterBuilder::new().from_rgb().to_hsv().build();
    let hsv = converter.convert(&rgb).unwrap();
    // HsvColor H = 0 S = 100 V = 100 A = 255
}

fn from_rgb_to_hsl()
{
    let rgb = RgbColor::new(255,0,0);
    let converter = ColorConverterBuilder::new().from_rgb().to_hsv().build();
    let hsv = converter.convert(&rgb).unwrap();
    // HslColor H = 0 S = 100 L = 50 A = 255
}
```

### HTML -> RGB or RGB -> HTML
```rust
extern crate irospace;
use irospace::{colors::Colors,
    RgbColor,HsvColor,HslColor,
    converter::*,
    ColorConverterBuilder};

fn from_html_to_rgb()
{
    let color_code = HtmlColorCode::new("#ff0000");
    let converter = ColorConverterBuilder::new().from_html().to_rgb().build();
    let rgb = converter.convert(&color_code).unwrap();
    println!("{}",rgb);
    // RgbColor R = 255 G = 0 B = 0 A = 255
}

fn from_rgb_to_html()
{
    let rgb = RgbColor::new(255,0,0);
    let converter = ColorConverterBuilder::new().from_rgb().to_html().build();
    let html = converter.convert(&rgb).unwrap();
    println!("{}",html.value_ref());
    // #ff0000
}

```


## 実行確認環境
* cargo 1.46.0 
* rustc 1.46.0

## License
[MIT License](LICENSE)

