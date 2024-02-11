use std::fs::File;
use printpdf::*;

pub struct Element {

}

impl Element {
    pub fn new(doc : &PdfDocumentReference, layer : &PdfLayerReference, text_array: Vec<String>, font_size:f32, x_axis: f32, y_axis:f32, font: &str)  {
            let path_to_font =  "fonts/".to_owned() + &*font+".ttf";
            let family = doc.add_external_font(File::open(path_to_font).unwrap()).unwrap();
            // layer.use_text(text, font_size.into(), Mm(x_axis.into()), Mm(y_axis.into()), &family)
            layer.begin_text_section();
     layer.set_line_height(14.0);
        layer.set_text_cursor(Mm(10.0), Mm(10.0));
            // write two lines (one line break)
        for text in text_array {
            layer.write_text(text, &family);
            layer.add_line_break();
        }
        layer.end_text_section();
    }
}


