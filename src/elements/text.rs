use std::fs::File;
use printpdf::*;

pub struct Text {
    text: String,
    font_family: IndirectFontRef,
    font_size : f64,
    x_axis: f32,
    y_axis: f32,

}

impl Text {
    pub fn new(doc : &PdfDocumentReference, text: &str, font_size:f64, x_axis: f32, y_axis:f32, font: &str) -> Self {

        if(font == "default"){
            let family =doc.add_builtin_font(BuiltinFont::TimesRoman).unwrap();
            Text {
                text: text.parse().unwrap(),
                font_family :family,
                font_size,
                x_axis,
                y_axis,
            }

        }else{
            let path_to_font =  "assets/fonts/".to_owned() + &*font;
            let family = doc.add_external_font(File::open(path_to_font).unwrap()).unwrap();
            Text {
                text: text.parse().unwrap(),
                font_family :family,
                font_size,
                x_axis,
                y_axis,
            }
        }

    }


    pub fn add_text(&self, layer : &PdfLayerReference){
        layer.use_text(&self.text, self.font_size.into(), Mm(self.x_axis.into()), Mm(self.y_axis.into()), &self.font_family);
    }

}


