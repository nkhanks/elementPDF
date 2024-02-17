
use std::io::Read;
use base64::Engine;

use printpdf::*;
use printpdf::Font::BuiltinFont;



pub struct Element;

impl Element {
    pub  fn new(doc : &PdfDocumentReference, layer : &PdfLayerReference, text: &str, font_size:f32, x_axis: f32, y_axis:f32, font: IndirectFontRef)  {

        let new_y_axis =   297.0 - y_axis;

        //let path_to_font =  "./fonts/".to_owned() + &*font+".ttf";

      //  let mut font_file =File::open(path_to_font).unwrap();
      //  let bytes = fonts::get_font_byte(font);


        //let family = doc.add_external_font( font_file).unwrap();
        // let family =  doc.add_builtin_font(BuiltinFont::Courier).unwrap();
       // let family = doc.add_external_font( File::open(path_to_font)).unwrap();
        layer.use_text(text, font_size.into(), Mm(x_axis.into()), Mm(new_y_axis.into()), &font)

    }
}


