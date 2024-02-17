use std::fs::File;
use std::io::BufWriter;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use printpdf::{Error, IndirectFontRef, Mm, PdfDocument};
use serde::{Deserialize, Serialize};


pub mod text;
pub mod image;
pub mod rectangle;
pub mod paragraph;


pub struct Fonts {
    name: String,
    font: IndirectFontRef,
}

#[derive(Serialize, Deserialize)]
pub struct Elements {
    font_family: String,
    url: String,
    path:String,
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
    #[serde(default = "default_font")]
    font: String,
    x: f32,
    y: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParagraphElement {
    value: Vec<String>,
    font_size: f32,
    #[serde(default = "default_font")]
    font_family: String,
    x: f32,
    y: f32
}

fn default_font() -> String {
    "Regular".to_string()
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


pub async fn gen_pdf(json_str: &str) -> Result<Vec<u8>, Error> {



    let data: Elements = serde_json::from_str(json_str).unwrap();

    let (doc, page1, layer1) = PdfDocument::new("printpdf graphics test", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);


    let mut fonts:Vec<Fonts> =vec![];;

    for element in data.elements {
        match element {
            Element::Image(image_element) => {
                let image = BASE64_STANDARD.decode(image_element.value).unwrap();
                image::Element::new(&current_layer,image,&image_element.image_type,image_element.x,image_element.y,image_element.scale);

            }
            Element::Text(text_element) => {

                let mut use_font;
                if let Some(found_font) = fonts.iter().find(|font| font.name == text_element.font) {
                    use_font = found_font.font.clone();
                } else {
                    let font_url = format!("{}{}{}-{}.ttf",&data.url,&data.path,&data.font_family,&text_element.font);
                    let bytes = reqwest::get(font_url.clone())
                        .await.unwrap()
                        .bytes()
                        .await.unwrap();


                    let mut font_reader = std::io::Cursor::new(bytes);

                     use_font = doc.add_external_font(&mut font_reader).unwrap();

                    fonts.push(Fonts{
                        font: use_font.clone(),
                        name: text_element.font
                    });



                }




                text::Element::new(&doc,&current_layer,&text_element.value,text_element.font_size,text_element.x,text_element.y,use_font);
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

        }

    }

   // doc.save(&mut BufWriter::new(File::create("test_working.pdf").unwrap())).unwrap();
    doc.save_to_bytes()
    //let my_vec: Vec<u8> = vec![0, 9];

    //Ok(my_vec)
}