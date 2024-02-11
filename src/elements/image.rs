
mod elements;

use ::image::codecs::png::PngDecoder;
use printpdf::*;
use printpdf::Image;
use std::fs::File;
use std::io::{BufWriter, Cursor};
use std::iter::FromIterator;
use base64::prelude::*;
use ::image::codecs::jpeg::JpegDecoder;


use crate::elements::text::Text;


fn main()  {

    let (doc, page1, layer1) = PdfDocument::new("printpdf graphics test", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);


    let base64 = "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";
     let image  =  BASE64_STANDARD.decode(base64).unwrap();

    let cursor = Cursor::new(image);
    let decoder = JpegDecoder::new(cursor).unwrap();

    let mut image = Image::try_from(decoder).unwrap();

    image.image = remove_alpha_channel_from_image_x_object(image.image);

    let rotation_center_x = Px((image.image.width.0 as f32 / 2.0) as usize);
    let rotation_center_y = Px((image.image.height.0 as f32 / 2.0) as usize);

    // layer,
    image.add_to_layer(
        current_layer.clone(),
        ImageTransform {

            rotate: Some(ImageRotation {
                angle_ccw_degrees: 0.0,
                rotation_center_x,
                rotation_center_y,
            }),
            translate_x: Some(Mm(50.0)),
            translate_y: Some(Mm(50.0)),
            scale_x: Some(2.0),
            scale_y:  Some(2.0),
            ..Default::default()
        },
    );



    Text::new(&doc,&current_layer,"Test else",23.0,1.0,280.0,"liberationsans");


    doc.save(&mut BufWriter::new(File::create("test_working.pdf").unwrap())).unwrap();

}


pub fn remove_alpha_channel_from_image_x_object(image_x_object: ImageXObject) -> ImageXObject {
    if !matches!(image_x_object.color_space, ColorSpace::Rgba) {
        return image_x_object;
    };
    let ImageXObject {
        color_space,
        image_data,
        ..
    } = image_x_object;

    let new_image_data = image_data
        .chunks(4)
        .map(|rgba| {
            let [red, green, blue, alpha]: [u8; 4] = rgba.try_into().ok().unwrap();
            let alpha = alpha as f64 / 255.0;
            let new_red = ((1.0 - alpha) * 255.0 + alpha * red as f64) as u8;
            let new_green = ((1.0 - alpha) * 255.0 + alpha * green as f64) as u8;
            let new_blue = ((1.0 - alpha) * 255.0 + alpha * blue as f64) as u8;
            return [new_red, new_green, new_blue];
        })
        .collect::<Vec<[u8; 3]>>()
        .concat();

    let new_color_space = match color_space {
        ColorSpace::Rgba => ColorSpace::Rgb,
        ColorSpace::GreyscaleAlpha => ColorSpace::Greyscale,
        other_type => other_type,
    };

    ImageXObject {
        color_space: new_color_space,
        image_data: new_image_data,
        ..image_x_object
    }
}