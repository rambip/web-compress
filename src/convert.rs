use wasm_bindgen::Clamped;

use web_sys::{HtmlCanvasElement, ImageData};

use image::io::Reader as ImageReader;
use std::io::Cursor;

pub fn test_display_image(image_data: &[u8], quality: u32, canvas: &HtmlCanvasElement) {
    let ctx = super::canvas::get_ctx(canvas);

    // read and convert image
    let img = ImageReader::new(Cursor::new(image_data)).with_guessed_format().unwrap().decode().unwrap();
    let mut img2 = img.resize(canvas.width(), canvas.height(), image::imageops::FilterType::CatmullRom)
        .into_rgba8();

    // TODO: encode as jpeg and show


    let width = img2.width();
    let height = img2.height();

    let new_img_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&mut img2),
        width, height
    ).unwrap();

    // draw image on the canvas
    ctx.put_image_data(&new_img_data, 0.0, 0.0)
        .expect("cannot put image data on canvas");
}
