use std::fs::File;
use printpdf::*;

pub struct Element {

}

impl Element {
    pub fn new(doc : &PdfDocumentReference, layer : &PdfLayerReference, text: &str, font_size:f32, x_axis: f32, y_axis:f32, font: &str)  {
        if(font == "default"){
            let family =doc.add_builtin_font(BuiltinFont::TimesRoman).unwrap();
            layer.use_text(text, font_size.into(), Mm(x_axis.into()), Mm(y_axis.into()), &family)
        }else{
            let path_to_font =  "fonts/".to_owned() + &*font+".ttf";
            let family = doc.add_external_font(File::open(path_to_font).unwrap()).unwrap();
            layer.use_text(text, font_size.into(), Mm(x_axis.into()), Mm(y_axis.into()), &family)
        }
    }
}


