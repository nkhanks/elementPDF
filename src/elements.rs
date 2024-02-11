use std::fs::File;
use std::io::BufWriter;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use printpdf::{Mm, PdfDocument};
use serde::{Deserialize, Serialize};


pub mod text;
pub mod image;
mod rectangle;
mod paragraph;


#[derive(Serialize, Deserialize)]
pub struct Elements {
    font_family: String,
    elements: Vec<Element>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "element")]
pub enum Element {
    Image(ImageElement),
    Text(TextElement),
    Rectangle(RectangleElement),
    Paragraph(ParagraphElement)
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageElement {
    value: String,
    image_type:  String,
    scale: f32,
    x: f32,
    y: f32
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TextElement {
    value: String,
    font_size: f32,
    #[serde(default = "default_font_family")]
    font_family: String,
    x: f32,
    y: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParagraphElement {
    value: Vec<String>,
    font_size: f32,
    #[serde(default = "default_font_family")]
    font_family: String,
    x: f32,
    y: f32
}

fn default_font_family() -> String {
    "liberationsans".to_string()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RectangleElement {
    width: f32,
    height: f32,
    thickness: f32,
    fill: String,
    outline: String,
    x : f32,
    y: f32
}

pub fn gen_pdf(json_str: &str)  {


    let data: Elements = serde_json::from_str(json_str).unwrap();

    let (doc, page1, layer1) = PdfDocument::new("printpdf graphics test", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);



    for element in data.elements {
        match element {
            Element::Image(image_element) => {
                let image = BASE64_STANDARD.decode(image_element.value).unwrap();
                image::Element::new(&current_layer,image,&image_element.image_type,image_element.x,image_element.y,image_element.scale);

            }
            Element::Text(text_element) => {
                if(&data.font_family == &text_element.font_family){
                    text::Element::new(&doc,&current_layer,&text_element.value,text_element.font_size,text_element.x,text_element.y,&data.font_family);

                }else{
                    text::Element::new(&doc,&current_layer,&text_element.value,text_element.font_size,text_element.x,text_element.y,&text_element.font_family);

                }

            }
            Element::Rectangle(rectangle_element) => {
                rectangle::Element::new(
                    &current_layer,
                    rectangle_element.width,
                    rectangle_element.height,
                    rectangle_element.x,
                    rectangle_element.y,
                    rectangle_element.thickness,
                    rectangle_element.fill,
                    rectangle_element.outline,
                );

            }
            Element::Paragraph(paragraph_element) => {
                paragraph::Element::new(
                    &doc,
                    &current_layer,
                    paragraph_element.value,
                    paragraph_element.font_size,
                    paragraph_element.x,
                    paragraph_element.y,
                    &paragraph_element.font_family);

            }
            _ => {}
        }

    }





    doc.save(&mut BufWriter::new(File::create("test_working.pdf").unwrap())).unwrap();

}