use std::fs::File;
use printpdf::*;

pub struct Element;

impl Element {
    pub fn new(doc : &PdfDocumentReference, layer : &PdfLayerReference, text: &str, font_size:f32, x_axis: f32, y_axis:f32, font: &str)  {

        let new_y_axis =   297.0 - y_axis;

        let path_to_font =  "fonts/".to_owned() + &*font+".ttf";
        let family = doc.add_external_font(File::open(path_to_font).unwrap()).unwrap();
        layer.use_text(text, font_size.into(), Mm(x_axis.into()), Mm(new_y_axis.into()), &family)

    }
}


