
use printpdf::*;
use printpdf::Image;
use std::io::{Cursor};
use ::image::codecs::jpeg::JpegDecoder;


pub struct Element {

}

impl Element {
    pub fn new(layer: &PdfLayerReference,image_byte: Vec<u8>,image_type: &str,x_axis: f32,y_axis : f32,scale: f32)  {
        let cursor = Cursor::new(image_byte);

        let decoder = JpegDecoder::new(cursor).unwrap();

        let mut image = Image::try_from(decoder).unwrap();

        image.image = remove_alpha_channel_from_image_x_object(image.image);

        let rotation_center_x = Px((image.image.width.0 as f32 / 2.0) as usize);
        let rotation_center_y = Px((image.image.height.0 as f32 / 2.0) as usize);

        let new_y_axis =  297.0 - y_axis;
        // layer,
        image.add_to_layer(
            layer.clone(),
            ImageTransform {
                rotate: Some(ImageRotation {
                    angle_ccw_degrees: 0.0,
                    rotation_center_x,
                    rotation_center_y,
                }),
                translate_x: Some(Mm(x_axis)),
                translate_y: Some(Mm(new_y_axis)),
                scale_x: Some(scale),
                scale_y:  Some(scale),
                ..Default::default()
            },
        );
    }
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