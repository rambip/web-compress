use web_sys::CanvasRenderingContext2d;

use image::io::Reader as ImageReader;
use image::{RgbaImage, DynamicImage};
use image::codecs::jpeg::{JpegEncoder, JpegDecoder};

use std::io::Cursor;
use std::io::Write;

use zip::ZipWriter;
use zip::write::FileOptions;

pub fn test_display_image(image: &RgbaImage, quality: u8, ctx: &CanvasRenderingContext2d) -> usize {
    let (canvas_width, canvas_height) = super::canvas::get_size(ctx);

    // buffer in memory where the jpeg converted image will be
    let mut jpeg_buf = Vec::with_capacity(image.len());

    JpegEncoder::new_with_quality(&mut jpeg_buf, quality).encode_image(image)
        .expect("can't create jpeg");

    let size_result = jpeg_buf.len();


    // re-read the jpeg image to an image
    let mut decoder = JpegDecoder::new(Cursor::new(jpeg_buf)).unwrap();
    decoder.scale(canvas_width as u16 *2 / 3, canvas_height as u16 * 2 /3).unwrap();
    let result = DynamicImage::from_decoder(decoder).unwrap().into_rgba8();

    // display the jpeg image
    super::canvas::draw_image(result.width(), result.height(), &result, ctx);
    
    size_result
}

pub fn read_image(d: &[u8]) -> RgbaImage {
    ImageReader::new(Cursor::new(d)).with_guessed_format().unwrap().decode().unwrap().into_rgba8()
}


pub fn convert_and_zip_images(images: &Vec<(String, super::FileData)>, quality: u8) -> Vec<u8> {
    let mut result = Vec::new();
    {
        let mut zip = ZipWriter::new(Cursor::new(&mut result));

        let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);

        let mut jpeg_buf = Vec::new();

        for (name, image) in images {
            jpeg_buf.clear();
            match image {
                super::FileData::Encoded(d) => {
                    JpegEncoder::new_with_quality(&mut jpeg_buf, quality).encode_image(&read_image(d))
                        .expect("can't create jpeg");
                }
                super::FileData::Decoded(pixels) => {
                    JpegEncoder::new_with_quality(&mut jpeg_buf, quality).encode_image(pixels)
                        .expect("can't create jpeg");
                }
            };

            let base_name = {
                let mut parts : Vec<&str> = name.split('.').into_iter().collect();
                if parts.len() > 1 {
                    parts.pop();
                    parts.concat().to_string()
                }
                else {
                    name.to_string()
                }
            };

            zip.start_file(format!("{}.jpg", base_name), options)
                .unwrap();

            zip.write(&jpeg_buf)
                .unwrap();
        }

        zip.finish().expect("cannot create zip");
    }

    result
}
