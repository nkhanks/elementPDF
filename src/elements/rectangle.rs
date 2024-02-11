use std::fs::File;
use printpdf::*;
use printpdf::lopdf::Outline;
use printpdf::path::{PaintMode, WindingOrder};

pub struct Element;

impl Element {
    pub fn new( layer : &PdfLayerReference,width: f32,height: f32,x: f32,y : f32,thickness:f32,fill: String,outline: String,)  {
        // Quadratic shape. The "false" determines if the next (following)
        // point is a bezier handle (for curves)
        //
        // If you want holes, use WindingOrder::EvenOdd
        let points1 = vec![
            (Point::new(Mm(x), Mm(y)), false),
            (Point::new(Mm(x+width), Mm(y)), false),
            (Point::new(Mm(x+width), Mm(y+height)), false),
            (Point::new(Mm(x), Mm(y+height)), false),
        ];


        let line1 = Polygon {
            rings: vec![points1],
            mode: PaintMode::FillStroke,
            winding_order: WindingOrder::NonZero,
        };

        let fill_color = Color::Rgb(Self::hex_to_rgb(&fill));
        let outline_color = Color::Rgb(Self::hex_to_rgb(&outline));
        let mut dash_pattern = LineDashPattern::default();
        dash_pattern.dash_1 = Some(20);

        layer.set_fill_color(fill_color);
        layer.set_outline_color(outline_color);
        layer.set_outline_thickness(thickness);

        // Draw first line
        layer.add_polygon(line1);
        let fill_color = Color::Rgb(Self::hex_to_rgb("#000000"));
        layer.set_fill_color(fill_color);
    }


    // Convert hexadecimal color to RGB ratios
    fn hex_to_rgb(hex: &str) -> Rgb {
        if hex.len() != 7 || !hex.starts_with("#") {
            return  Rgb::new(0 as f32,0 as f32, 0 as f32, None)
        }

        let r = u8::from_str_radix(&hex[1..3], 16).ok().unwrap() as f64 / 256.0;
        let g = u8::from_str_radix(&hex[3..5], 16).ok().unwrap() as f64 / 256.0;
        let b = u8::from_str_radix(&hex[5..7], 16).ok().unwrap() as f64 / 256.0;

        Rgb::new(r as f32, g as f32, b as f32, None)
    }


}


